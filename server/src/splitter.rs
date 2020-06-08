use std::time::Duration;
use futures::channel::mpsc::{self, UnboundedSender as Sender, UnboundedReceiver as Receiver};
use async_std::io::Read;
use std::pin::Pin;
use std::task::Poll;
use async_std::task::Context;
use async_std::prelude::StreamExt;

use std::sync::{Arc, Mutex};
use crate::recent_cache::RecentCache;

const SILENCE_POWER_THRESHOLD: f64 = 1_000_000_000_000.0;
const METADATA_SENT_ON_INITIAL_LOAD: usize = 400;
const RECONNECT_WAIT: Duration = Duration::from_secs(5);

#[derive(Clone)]
pub struct Splitter {
    metadata: RecentCache<AudioMetadata>,
}

#[derive(Hash, Debug, Clone, Copy)]
pub struct AudioId(u64);

#[derive(Clone, Debug)]
pub struct AudioMetadata {
    timestamp: u64,
    channel: String,
    id: AudioId,
}

pub enum AudioStream {
    File,
    Livestream,
}

impl Read for AudioStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8]
    ) -> Poll<async_std::io::Result<usize>> {
        unimplemented!()
    }
}

impl Splitter {
    pub fn new() -> Self {
        Self {
            metadata: RecentCache::new(METADATA_SENT_ON_INITIAL_LOAD),
        }
    }

    pub fn add_source(&self, channel_name: String, url: String) {
        let this = self.clone();
        std::thread::spawn(move || {
            loop {
                let resp = reqwest::blocking::get(&url).unwrap();
                let mut decoder = simplemad::Decoder::decode(resp).unwrap();

                for frame in decoder {
                    match frame {
                        Err(e) => println!("[{}] mp3 decoding error: {:?}", &channel_name, e),
                        Ok(frame) => {
                            this.handle_frame(&channel_name, &url, frame);
                        }
                    }
                }

                println!("[{}] disconnected from {}, will attempt to reconnect in {} seconds...", &channel_name, &url, RECONNECT_WAIT.as_secs());
                std::thread::sleep(RECONNECT_WAIT);
            }
        });
    }

    fn handle_frame(&self, channel_name: &str, url: &str, frame: simplemad::Frame) {
    }

    pub fn recent_stream(&self) -> Receiver<AudioMetadata> {
        self.metadata.get_stream()
    }

    pub fn stream_audio(&self, id: AudioId) -> AudioStream {
        unimplemented!()
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