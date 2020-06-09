use async_std::fs::File;
use async_std::prelude::*;
use async_std::prelude::*;
use async_std::stream::Stream;
use async_std::task::Context;
use futures::channel::mpsc::{self, UnboundedReceiver as Receiver, UnboundedSender as Sender};
use std::sync::mpsc as sync_mpsc;
use std::pin::Pin;
use std::task::Poll;
use std::time::Duration;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use hound::WavWriter;

use crate::silence_gate::SilenceGate;
use crate::recent_cache::RecentCache;
use std::sync::Arc;
use async_std::sync::Mutex;

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
    },
}

#[derive(Clone, Debug)]
pub struct AudioMetadata {
    timestamp: u64,
    channel: String,
    id: AudioId,
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioId(u64);

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
            } => {
                if current_chunk.is_none() {
                    match Stream::poll_next(Pin::new(new_chunks), cx) {
                        // waiting for new chunks
                        Poll::Pending => return Poll::Pending,
                        // end of stream; report zero more chunks
                        Poll::Ready(None) => return Poll::Ready(Ok(0)),
                        // another item is ready, load it up
                        Poll::Ready(Some(item)) => {
                            *current_index = 0;
                            *current_chunk = Some(item);
                        },
                    }
                }
                let mut total_written = 0;
                loop {
                    let chunk = match current_chunk {
                        Some(v) => v,
                        None => break,
                    };

                    let left_in_buf = buf.len() - total_written;
                    let left_in_current_chunk = chunk.len() - *current_index;
                    let write_len = left_in_buf.min(left_in_current_chunk);

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

    pub fn get_audio_input(&self, channel_name: String) -> sync_mpsc::Sender<(Vec<i32>, u32)> {
        let (sender, receiver) = sync_mpsc::channel::<(Vec<i32>, u32)>();
        let this = self.clone();
        std::thread::spawn(move || {
            let mut current_message_writer: Option<WavWriter<_>> = None;
            let mut silence_gate = SilenceGate::new();
            while let Ok((data, sample_rate)) = receiver.recv() {
                for chunk in data.chunks(10_000) {
                    silence_gate.add_sound(&chunk);
                    match (silence_gate.is_open(), current_message_writer.take()) {
                        (false, Some(mut writer)) => {
                            // close the current message after sending this data
                            for sample in chunk {
                                writer.write_sample(*sample);
                            }
                        }
                        (true, Some(mut writer)) => {
                            // continue current message
                            for sample in chunk {
                                writer.write_sample(*sample);
                            }
                            current_message_writer = Some(writer);
                        }
                        (false, None) => {
                            // do nothing
                        }
                        (true, None) => {
                            // open a new message
                            let timestamp = (SystemTime::now()
                                                            .duration_since(UNIX_EPOCH)
                                                            .expect("Time went backwards")
                                                            .as_millis()
                                                            as u64);
                            // let new_stream = RecentCache::new(None);
                            let id = this.next_id.fetch_add(1, Ordering::SeqCst);
                            let new_writer = WavWriter::create(
                                format!("{}.wav", id),
                                hound::WavSpec {
                                    channels: 1,
                                    sample_rate,
                                    bits_per_sample: 32,
                                    sample_format: hound::SampleFormat::Int,
                                }).unwrap();
                            // new_stream.send_item(chunk.to_vec());
                            // let livestreams = this.livestreams.clone();
                            // async_std::task::block_on(async move {
                            //     livestreams.lock().await.insert(AudioId(id), new_stream.clone());
                            // });
                            let metadata = AudioMetadata {
                                timestamp,
                                channel: channel_name.clone(),
                                id: AudioId(id),
                            };
                            this.metadata.send_item(metadata);
                            current_message_writer = Some(new_writer);
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
                })
            }
            None => {
                // no longer streaming live, serve from filesystem
                match async_std::fs::File::open(format!("{}.wav", id.0)).await {
                    Ok(v) => Some(AudioStream::File(v)),
                    Err(_) => None,
                }
            }
        }
    }
}

// fn main() {
//     let mut writer_opt = None;
//     let mut n = 0;
//     let mut file_len = 0;
//     loop {
//         match decoder.get_frame() {
//             Err(e) => println!("Error: {:?}", e),
//             Ok(frame) => {
//                 let writer = writer_opt.get_or_insert_with(|| {
//                     let spec = ;
//                     n += 1;
//                     println!("== file {} ==", n);
//                     hound::WavWriter::create(&format!("{}.wav", n), spec).unwrap()
//                 });
//                 let sum = 0.0;

//                 let mut total_power = 0.0;
//                 for sample in &frame.samples[0] {
//                     let s = sample.to_i32();
//                     writer.write_sample(s);
//                     total_power += (s as f64) * (s as f64);
//                 }
//                 let average_power = power / frame.samples[0].len() as f64;
//                 println!("[{}] {:?}", frame.position.as_secs(), average_power);

//                 if frame.position.as_secs() > 10 {
//                     break;
//                 }
//             },
//         }
//     }
// }
