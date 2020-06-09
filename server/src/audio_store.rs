use async_std::fs::File;
use async_std::prelude::*;
use async_std::prelude::*;
use async_std::stream::Stream;
use async_std::task::Context;
use futures::channel::mpsc::{self, UnboundedReceiver as Receiver, UnboundedSender as Sender};
use std::pin::Pin;
use std::task::Poll;
use std::time::Duration;
use std::collections::HashMap;

use crate::recent_cache::RecentCache;
use std::sync::Arc;
use async_std::sync::Mutex;

const SILENCE_POWER_THRESHOLD: f64 = 1_000_000_000_000.0;

/// Splits multiple streams of wav audio into non-silent chunks, saves to disk,
/// serves audio files based on id.
#[derive(Clone)]
pub struct AudioStore {
    livestreams: Arc<Mutex<HashMap<AudioId, RecentCache<Vec<u8>>>>>,
    metadata: RecentCache<AudioMetadata>,
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
        Self {
            livestreams: Arc::new(Mutex::new(HashMap::new())),
            metadata,
        }
    }

    pub fn add_audio(&self, channel_name: &str, data: Vec<i32>) {
        // metadata_cache.send_item...()
        unimplemented!()
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
//                     let spec = hound::WavSpec {
//                         channels: 1,
//                         sample_rate: frame.sample_rate,
//                         bits_per_sample: 32,
//                         sample_format: hound::SampleFormat::Int,
//                     };
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
