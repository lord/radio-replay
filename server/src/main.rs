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


fn main() {
    let resp = reqwest::blocking::get("https://broadcastify.cdnstream1.com/32890").unwrap();
    let mut decoder = simplemad::Decoder::decode(resp).unwrap();
    let mut samples = vec![];
    let mut sample_rate = 0;
    loop {
        match decoder.get_frame() {
            Err(e) => println!("Error: {:?}", e),
            Ok(frame) => {
                sample_rate = frame.sample_rate;
                if frame.position.as_secs() > 10 {
                    for sample in &frame.samples[0] {
                        samples.push(sample.to_i32())
                    }
                }

                if frame.position.as_secs() > 20 {
                    break;
                }
            },
        }
    }

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sample.wav", spec).unwrap();
    for sample in samples {
        writer.write_sample(sample).unwrap();
    }
}