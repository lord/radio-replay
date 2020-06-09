use async_std::fs::File;
use async_std::prelude::*;
use async_std::stream::Stream;
use async_std::task::Context;
use futures::channel::mpsc::{self, UnboundedReceiver as Receiver, UnboundedSender as Sender};
use hound::WavWriter;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::mpsc as sync_mpsc;
use std::task::Poll;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::recent_cache::RecentCache;
use crate::silence_gate::SilenceGate;
use crate::encoder::Encoder;
use async_std::sync::Mutex;
use std::sync::Arc;

/// Splits multiple streams of wav audio into non-silent chunks, saves to disk,
/// serves audio files based on id.
#[derive(Clone)]
pub struct AudioStore {
    livestreams: Arc<Mutex<HashMap<AudioId, RecentCache<Vec<u8>>>>>,
    metadata: RecentCache<AudioMetadata>,
    next_id: Arc<AtomicU64>,
}

pub enum AudioStream {
    File(File),
    Livestream {
        new_chunks: Receiver<Vec<u8>>,
        current_chunk: Option<Vec<u8>>,
        current_index: usize,
        closed: bool,
    },
}

#[derive(Clone, Debug)]
pub struct AudioMetadata {
    pub timestamp: u64,
    pub channel: String,
    pub id: AudioId,
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioId(pub u64);

impl async_std::io::Read for AudioStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<async_std::io::Result<usize>> {
        match &mut self.get_mut() {
            AudioStream::File(file) => Pin::new(file).poll_read(cx, buf),
            AudioStream::Livestream {
                new_chunks,
                current_chunk,
                current_index,
                closed,
            } => {
                if *closed {
                    return Poll::Ready(Ok(0));
                }
                if current_chunk.is_none() {
                    match Stream::poll_next(Pin::new(new_chunks), cx) {
                        // waiting for new chunks
                        Poll::Pending => return Poll::Pending,
                        // end of stream; report zero more chunks
                        Poll::Ready(None) => {
                            println!("closed!");
                            *closed = true;
                            return Poll::Ready(Ok(0))
                        },
                        // another item is ready, load it up
                        Poll::Ready(Some(item)) => {
                            *current_index = 0;
                            *current_chunk = Some(item);
                        }
                    }
                }
                let mut total_written = 0;
                let mut i = 0;
                loop {
                    i += 1;
                    let chunk = match current_chunk {
                        Some(v) => v,
                        None => break,
                    };

                    let left_in_buf = buf.len() - total_written;
                    let left_in_current_chunk = chunk.len() - *current_index;
                    let write_len = left_in_buf.min(left_in_current_chunk);

                    if left_in_buf == 0 {
                        break
                    }

                    buf[total_written..(total_written + write_len)]
                        .copy_from_slice(&chunk[*current_index..(*current_index + write_len)]);
                    *current_index += write_len;
                    total_written += write_len;

                    if *current_index == chunk.len() {
                        *current_index = 0;
                        *current_chunk = match Stream::poll_next(Pin::new(new_chunks), cx) {
                            Poll::Ready(Some(item)) => Some(item),
                            _ => None,
                        }
                    }
                }

                Poll::Ready(Ok(total_written))
            }
        }
    }
}

impl AudioStore {
    pub fn new(metadata: RecentCache<AudioMetadata>) -> Self {
        // TODO load next_id from filesystem
        Self {
            livestreams: Arc::new(Mutex::new(HashMap::new())),
            metadata,
            next_id: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn get_audio_input(&self, channel_name: String) -> sync_mpsc::Sender<(Vec<i16>, u32)> {
        let (sender, receiver) = sync_mpsc::channel::<(Vec<i16>, u32)>();
        let this = self.clone();
        std::thread::spawn(move || {
            let mut current_message_writer: Option<(AudioId, Encoder<_>)> = None;
            let mut silence_gate = SilenceGate::new();
            while let Ok((data, sample_rate)) = receiver.recv() {
                for chunk in data.chunks(10_000) {
                    silence_gate.add_sound(&chunk);
                    match (silence_gate.is_open(), current_message_writer.take()) {
                        (false, Some((id, mut writer))) => {
                            // close the current message after sending this data
                            writer.add_pcm(chunk);
                            let this2 = this.clone();
                            async_std::task::spawn(async move {
                                let mut file = File::create(format!("{}.mp3", id.0)).await.unwrap();
                                let mut stream = this2.livestreams.lock().await.remove(&id).unwrap().get_stream();
                                while let Some(buf) = stream.next().await {
                                    file.write_all(&buf).await;
                                }
                                println!("file finished writing to disk");
                            });
                        }
                        (true, Some((id, mut writer))) => {
                            // continue current message
                            writer.add_pcm(chunk);
                            current_message_writer = Some((id, writer));
                        }
                        (false, None) => {
                            // do nothing
                        }
                        (true, None) => {
                            // open a new message
                            let timestamp = (SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("Time went backwards")
                                .as_millis() as u64);
                            let new_stream = RecentCache::new(None);
                            let id = this.next_id.fetch_add(1, Ordering::SeqCst);
                            println!("creating {}", id);
                            let mut new_writer = Encoder::new(new_stream.clone(), sample_rate);
                            new_writer.add_pcm(chunk);
                            current_message_writer = Some((AudioId(id), new_writer));
                            let this2 = this.clone();
                            let metadata = AudioMetadata {
                                timestamp,
                                channel: channel_name.clone(),
                                id: AudioId(id),
                            };
                            async_std::task::spawn(async move {
                                this2.livestreams.lock().await.insert(AudioId(id), new_stream.clone());
                                this2.metadata.send_item(metadata);
                            });
                        }
                    }
                }
            }
        });
        sender
    }

    pub async fn get_stream(&self, id: AudioId) -> Option<AudioStream> {
        match self.livestreams.lock().await.get(&id) {
            Some(livestream_cache) => {
                // still streaming live, hook up audio stream to that
                let new_chunks = livestream_cache.get_stream();
                Some(AudioStream::Livestream {
                    new_chunks,
                    current_chunk: None,
                    current_index: 0,
                    closed: false,
                })
            }
            None => {
                // no longer streaming live, serve from filesystem
                match async_std::fs::File::open(format!("{}.mp3", id.0)).await {
                    Ok(v) => Some(AudioStream::File(v)),
                    Err(_) => None,
                }
            }
        }
    }
}
