use async_std::prelude::StreamExt;
use async_std::task::Context;
use futures::channel::mpsc::{self, UnboundedReceiver as Receiver, UnboundedSender as Sender};
use std::pin::Pin;
use std::task::Poll;
use std::time::Duration;

use crate::recent_cache::RecentCache;
use std::sync::{Arc, Mutex};

use crate::audio_store::{AudioId, AudioMetadata, AudioStore, AudioStream};

const METADATA_SENT_ON_INITIAL_LOAD: usize = 400;
const RECONNECT_WAIT: Duration = Duration::from_secs(5);

pub struct AudioService {
    metadata: RecentCache<AudioMetadata>,
    store: AudioStore,
}

impl AudioService {
    pub fn new() -> AudioService {
        let metadata = RecentCache::new(Some(METADATA_SENT_ON_INITIAL_LOAD));
        AudioService {
            metadata: metadata.clone(),
            store: AudioStore::new(metadata),
        }
    }

    pub fn add_source(&self, channel_name: String, url: String) {
        let audio_in = self.store.get_audio_input(&channel_name);
        std::thread::spawn(move || loop {
            let resp = reqwest::blocking::get(&url).unwrap();
            let mut decoder = simplemad::Decoder::decode(resp).unwrap();

            for frame in decoder {
                match frame {
                    Err(e) => println!("[{}] mp3 decoding error: {:?}", &channel_name, e),
                    Ok(frame) => {
                        audio_in.unbounded_send(frame.samples[0].iter().map(|v| v.to_i32()).collect());
                    }
                }
            }

            println!(
                "[{}] disconnected from {}, will attempt to reconnect in {} seconds...",
                &channel_name,
                &url,
                RECONNECT_WAIT.as_secs()
            );
            std::thread::sleep(RECONNECT_WAIT);
        });
    }

    pub fn recent_stream(&self) -> Receiver<AudioMetadata> {
        self.metadata.get_stream()
    }

    pub async fn stream_audio(&self, id: AudioId) -> Option<AudioStream> {
        self.store.get_stream(id).await
    }
}
