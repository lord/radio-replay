// const SILENCE_POWER_THRESHOLD: f64 = 1_000_000_000_000.0;

// use std::time::Duration;
// use futures::channel::mpsc::{self, UnboundedSender as Sender, UnboundedReceiver as Receiver};
// use async_std::io::Read;
// use std::pin::Pin;
// use std::task::Poll;
// use async_std::task::Context;
// use async_std::prelude::StreamExt;

// use std::sync::{Arc, Mutex};

// pub struct Splitter {
//     recent_stream_sender: Sender<Sender<AudioMetadata>>
//     recent_metadatas: Arc<Mutex<Vec<AudioMetadata>>>,
// }

// pub struct AudioId(u64);

// pub struct AudioMetadata {
//     timestamp: u64,
//     channel: String,
//     id: AudioId,
// }

// pub enum AudioStream {
//     File,
//     Livestream,
// }

// impl Read for AudioStream {
//     fn poll_read(
//         self: Pin<&mut Self>,
//         cx: &mut Context,
//         buf: &mut [u8]
//     ) -> Poll<async_std::io::Result<usize>> {
//         unimplemented!()
//     }
// }

// impl Splitter {
//     pub fn new() -> Self {
//         let (sender, receiver) = mpsc::unbounded();
//         std::thread::spawn(move || {
//             Self::metadata_sender_thread(receiver);
//         });
//         Self {
//             recent_stream_sender: sender,
//         }
//     }

//     pub fn metadata_sender_thread(mut receiver: Receiver<Sender<AudioMetadata>>) {
//         while let Some(metadata_sender) = async_std::task::block_on(receiver.next()) {
//             // TODO FETCH RECENT METADATA UNDER MUTEX
//             // TODO SEND RECENT METADATA DOWN THE PIPE!!
//         }
//     }

//     pub fn add_source(&self, url: &str) {
//         let url = url.to_string();
//         // TODO spawn new thread
//         unimplemented!()
//     }

//     pub fn recent_stream(&self) -> Receiver<AudioMetadata> {
//         let (sender, receiver) = mpsc::unbounded();
//         self.recent_stream_sender.unbounded_send(sender).unwrap();
//         receiver
//     }

//     pub fn stream_audio(&self, id: AudioId) -> AudioStream {
//         unimplemented!()
//     }
// }

// // fn main() {
// //     let resp = reqwest::blocking::get("http://scanner.fuck12.tech:8000/nypd-cw1").unwrap();
// //     let mut decoder = simplemad::Decoder::decode(resp).unwrap();
// //     let mut writer_opt = None;
// //     let mut n = 0;
// //     let mut file_len = 0;
// //     loop {
// //         match decoder.get_frame() {
// //             Err(e) => println!("Error: {:?}", e),
// //             Ok(frame) => {
// //                 let writer = writer_opt.get_or_insert_with(|| {
// //                     let spec = hound::WavSpec {
// //                         channels: 1,
// //                         sample_rate: frame.sample_rate,
// //                         bits_per_sample: 32,
// //                         sample_format: hound::SampleFormat::Int,
// //                     };
// //                     n += 1;
// //                     println!("== file {} ==", n);
// //                     hound::WavWriter::create(&format!("{}.wav", n), spec).unwrap()
// //                 });
// //                 let sum = 0.0;

// //                 let mut total_power = 0.0;
// //                 for sample in &frame.samples[0] {
// //                     let s = sample.to_i32();
// //                     writer.write_sample(s);
// //                     total_power += (s as f64) * (s as f64);
// //                 }
// //                 let average_power = power / frame.samples[0].len() as f64;
// //                 println!("[{}] {:?}", frame.position.as_secs(), average_power);

// //                 if frame.position.as_secs() > 10 {
// //                     break;
// //                 }
// //             },
// //         }
// //     }
// // }