#![recursion_limit = "1024"]

mod audio_service;
mod audio_store;
mod encoder;
mod recent_cache;
mod silence_gate;

use async_std::prelude::StreamExt;
use regex::Regex;
use async_std::future::timeout;

use std::time::Duration;
static EVENTSTREAM_PING_TIMEOUT: Duration = Duration::from_secs(15);

lazy_static::lazy_static! {
    static ref MP3_FILENAME: Regex = Regex::new("(.*)\\.mp3$").unwrap();
}

async fn serve_audio(
    srv: audio_service::AudioService,
    req: tide::Request<()>,
) -> Result<tide::Response, tide::Error> {
    let bad_file = |msg| tide::Error::from_str(tide::StatusCode::NotFound, msg);
    let text: String = req
        .param("audio_id")
        .map_err(|_| bad_file("unknown parameter"))?;
   let audio_text = &MP3_FILENAME
        .captures(&text)
        .ok_or(bad_file("not valid mp3"))?[1];
    let audio_id = audio_text
        .parse()
        .map_err(|_| bad_file("not valid number"))?;
    let stream = srv
        .stream_audio(audio_store::AudioId(audio_id))
        .await
        .ok_or(bad_file("couldn't find that stream id"))?;
    let mut resp = tide::Response::new(tide::StatusCode::Ok);
    resp.set_content_type("audio/mpeg".parse::<tide::http::Mime>().unwrap());
    resp.insert_header("content-disposition", "attachment");
    resp.set_body(tide::Body::from_reader(
        async_std::io::BufReader::new(stream),
        None,
    ));
    Ok(resp)
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let audio_service = audio_service::AudioService::new();
    audio_service.add_source(
        "your channel name 1".to_string(),
        "http://example.com/my_audio_stream".to_string(),
    );
    audio_service.add_source(
        "your channel name 2".to_string(),
        "http://example.com/my_audio_stream_2".to_string(),
    );
    let mut app = tide::new();
    let srv = audio_service.clone();
    app.at("/replay").get(|_| async move {
        let mut resp = tide::Response::new(200);
        resp.set_body(tide::Body::from_file("../client/build/index.html").await?);
        resp.set_content_type("text/html".parse::<tide::http::Mime>().unwrap());
        Ok(resp)
    });
    app.at("/audio/:audio_id")
        .get(move |req| serve_audio(srv.clone(), req));
    let srv = audio_service.clone();
    app.at("/stream")
        .get(tide::sse::endpoint(move |_req, sender| {
            let srv = srv.clone();
            async move {
                let mut stream = srv.recent_stream();
                loop {
                    match timeout(EVENTSTREAM_PING_TIMEOUT, stream.next()).await {
                        Ok(Some(metadata)) =>  {
                            let json = format!(
                                "{{\"timestamp\":{},\"channel\":\"{}\",\"url\":\"/audio/{}.mp3\"}}",
                                metadata.timestamp, metadata.channel, metadata.id.0
                            );
                            sender.send("audio", json, None).await;
                        }
                        Ok(None) => break, // end event stream
                        Err(_) => {
                            // timeout, send update so eventstream doesn't close
                            sender.send("ping", "", None).await;
                        }
                    }
                }
                Ok(())
            }
        }));
    app.at("/").serve_dir("../client/build/")?;
    app.listen("localhost:8080").await?;
    Ok(())
}
