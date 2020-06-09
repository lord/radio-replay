#![recursion_limit = "1024"]

mod audio_service;
mod audio_store;
mod recent_cache;
mod silence_gate;
mod encoder;

use regex::Regex;
use async_std::prelude::StreamExt;

lazy_static::lazy_static! {
    static ref MP3_FILENAME: Regex = Regex::new("(.*)\\.mp3$").unwrap();
}

async fn serve_audio(srv: audio_service::AudioService, req: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    let bad_file = |msg| tide::Error::from_str(tide::StatusCode::NotFound, msg);
    let text: String = req.param("audio_id").map_err(|_| bad_file("unknown parameter"))?;
    let audio_text = &MP3_FILENAME.captures(&text).ok_or(bad_file("not valid mp3"))?[1];
    let audio_id = audio_text.parse().map_err(|_| bad_file("not valid number"))?;
    let stream = srv.stream_audio(audio_store::AudioId(audio_id)).await.ok_or(bad_file("couldn't find that stream id"))?;
    let mut resp = tide::Response::new(tide::StatusCode::Ok);
    resp.set_content_type("audio/mpeg".parse::<tide::http::Mime>().unwrap());
    resp.set_body(tide::Body::from_reader(async_std::io::BufReader::new(stream), None));
    Ok(resp)
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut audio_service = audio_service::AudioService::new();
    audio_service.add_source(
        "nypd-cw2".to_string(),
        "https://broadcastify.cdnstream1.com/32890".to_string(),
    );

    let mut app = tide::new();
    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.at("/").get(|_| async { Ok("Hello, world!") });
    let srv = audio_service.clone();
    app.at("/audio/:audio_id").get(move |req| serve_audio(srv.clone(), req));
    let srv = audio_service.clone();
    app.at("/stream").get(tide::sse::endpoint(move |_req, sender| {
        let srv = srv.clone();
        async move {
            let mut stream = srv.recent_stream();
            while let Some(metadata) = stream.next().await {
                let json = format!("{{\"timestamp\":{},\"channel\":\"{}\",\"url\":\"/audio/{}.mp3\"}}", metadata.timestamp, metadata.channel, metadata.id.0);
                sender.send("audio", json, None).await;
            }
            Ok(())
        }
    }));
    app.listen("localhost:8080").await?;
    Ok(())
}
