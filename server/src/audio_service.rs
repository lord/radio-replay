use std::time::Duration;
use futures::channel::mpsc::{self, UnboundedSender as Sender, UnboundedReceiver as Receiver};
use std::pin::Pin;
use std::task::Poll;
use async_std::task::Context;
use async_std::prelude::StreamExt;

use std::sync::{Arc, Mutex};
use crate::recent_cache::RecentCache;

use crate::audio_store::{AudioMetadata, AudioStream, AudioId, AudioStore};

const METADATA_SENT_ON_INITIAL_LOAD: usize = 400;
const RECONNECT_WAIT: Duration = Duration::from_secs(5);

#[derive(Clone)]
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
        let this = self.clone();
        std::thread::spawn(move || {
            loop {
                let resp = reqwest::blocking::get(&url).unwrap();
                let mut decoder = simplemad::Decoder::decode(resp).unwrap();

                for frame in decoder {
                    match frame {
                        Err(e) => println!("[{}] mp3 decoding error: {:?}", &channel_name, e),
                        Ok(frame) => {
                            this.store.add_audio(&channel_name, frame.samples[0].iter().map(|v| v.to_i32()).collect());
                        }
                    }
                }

                println!("[{}] disconnected from {}, will attempt to reconnect in {} seconds...", &channel_name, &url, RECONNECT_WAIT.as_secs());
                std::thread::sleep(RECONNECT_WAIT);
            }
        });
    }

    pub fn recent_stream(&self) -> Receiver<AudioMetadata> {
        self.metadata.get_stream()
    }

    pub fn stream_audio(&self, id: AudioId) -> AudioStream {
        self.store.get_stream(id)
    }
}
