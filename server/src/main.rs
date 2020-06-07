// #[async_std::main]
// async fn main() -> Result<(), std::io::Error> {
//     let mut app = tide::new();
//     app.at("/").get(|_| async { Ok("Hello, world!") });
//     app.at("/sse").get(tide::sse::endpoint(|_req, sender| async move {
//         sender.send("fruit", "banana", None).await;
//         loop {
//             async_std::task::sleep(std::time::Duration::from_secs(2)).await;
//             sender.send("fruit", "apple", None).await;
//         }
//     }));
//     app.listen("localhost:8080").await?;
//     Ok(())
// }

// const SILENCE_POWER_THRESHOLD: f64 = 1_000_000_000_000.0;

// use std::time::Duration;

// fn main() {
//     let resp = reqwest::blocking::get("http://scanner.fuck12.tech:8000/nypd-cw1").unwrap();
//     let mut decoder = simplemad::Decoder::decode(resp).unwrap();
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