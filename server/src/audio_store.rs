use std::time::Duration;
use futures::channel::mpsc::{self, UnboundedSender as Sender, UnboundedReceiver as Receiver};
use std::pin::Pin;
use std::task::Poll;
use async_std::task::Context;
use async_std::prelude::StreamExt;

use std::sync::{Arc, Mutex};
use crate::recent_cache::RecentCache;

const SILENCE_POWER_THRESHOLD: f64 = 1_000_000_000_000.0;
/// Splits multiple streams of wav audio into non-silent chunks, saves to disk,
/// serves audio files based on id.
#[derive(Clone)]
pub struct AudioStore {
}

pub enum AudioStream {
    File,
    Livestream,
}

#[derive(Clone, Debug)]
pub struct AudioMetadata {
    timestamp: u64,
    channel: String,
    id: AudioId,
}

#[derive(Hash, Debug, Clone, Copy)]
pub struct AudioId(u64);

impl async_std::io::Read for AudioStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8]
    ) -> Poll<async_std::io::Result<usize>> {
        unimplemented!()
    }
}

impl AudioStore {
    pub fn new(metadata_cache: RecentCache<AudioMetadata>) -> Self {
        // metadata_cache.send_item...()
        unimplemented!()
    }

    pub fn add_audio(&self, channel_name: &str, data: Vec<i32>) {
        unimplemented!()
    }

    pub fn get_stream(&self, id: AudioId) -> AudioStream {
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