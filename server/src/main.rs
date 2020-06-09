#![recursion_limit = "1024"]

mod audio_service;
mod audio_store;
mod recent_cache;
mod silence_gate;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut audio_service = audio_service::AudioService::new();
    audio_service.add_source(
        "nypd-cw2".to_string(),
        "http://scanner.fuck12.tech:8000/nypd-cw1".to_string(),
    );

    let mut app = tide::new();
    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.at("/audio").serve_dir("example/")?;
    app.at("/stream").get(tide::sse::endpoint(|_req, sender| async move {
        let mut i: u64 = 0;
        for _ in 0..100 {
            i += 1;
            sender.send("audio", format!("{{\"timestamp\":1591558692000,\"channel\":\"nyc-cw1\",\"url\":\"/audio/example.mp3?{}\"}}", i), None).await;
        }
        loop {
            async_std::task::sleep(std::time::Duration::from_secs(12)).await;
            i += 1;
            sender.send("audio", format!("{{\"timestamp\":1591558692000,\"channel\":\"nyc-cw1\",\"url\":\"/audio/example.mp3?{}\"}}", i), None).await;
        }
    }));
    app.listen("localhost:8080").await?;
    Ok(())
}
