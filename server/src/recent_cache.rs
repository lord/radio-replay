use std::time::Duration;
use futures::channel::mpsc::{self, UnboundedSender as Sender, UnboundedReceiver as Receiver};
use async_std::io::Read;
use std::pin::Pin;
use std::task::Poll;
use async_std::task::Context;
use async_std::prelude::StreamExt;
use std::sync::Arc;
use async_std::sync::Mutex;
use std::collections::VecDeque;
use futures::select;
use futures::future::FutureExt;

#[derive(Clone)]
struct RecentCache<T: Send + Clone + 'static> {
    new_messages: Sender<T>,
    new_senders: Sender<Sender<T>>,
}

impl <T: Send + Clone + 'static> RecentCache<T> {
    pub fn new(capacity: usize) -> Self {
        let (new_messages_tx, new_messages_rx) = mpsc::unbounded();
        let (new_senders_tx, new_senders_rx) = mpsc::unbounded();
        async_std::task::spawn(Self::handler_task(capacity, new_messages_rx, new_senders_rx));
        Self {
            new_messages: new_messages_tx,
            new_senders: new_senders_tx,
        }
    }

    async fn handler_task(capacity: usize, mut new_messages: Receiver<T>, mut new_senders: Receiver<Sender<T>>) {
        let mut recent_messages = VecDeque::with_capacity(capacity);
        let mut senders = Vec::new();
        loop {
            let mut next_msg = new_messages.next().fuse();
            let mut next_sender = new_senders.next().fuse();
            select! {
                next_msg = next_msg => {
                    let item = match next_msg {
                        Some(v) => v,
                        // TODO not sure if should break here
                        None => break,
                    };
                    while recent_messages.len() >= capacity-1 && capacity > 0 {
                        recent_messages.pop_front();
                    }
                    if capacity > 0 {
                        recent_messages.push_back(item.clone());
                    }
                    senders.retain(|sender: &Sender<T>| {
                        sender.unbounded_send(item.clone()).is_ok()
                    });
                }
                next_sender = next_sender => {
                    let sender = match next_sender {
                        Some(v) => v,
                        // TODO not sure if should break here
                        None => break,
                    };
                    for message in recent_messages.iter() {
                        let _ = sender.unbounded_send(message.clone());
                    }
                    senders.push(sender);
                }
            }
        }
    }

    /// Will send the last `capacity` messages down receiever, as well as any subsequent
    pub fn get_stream(&self) -> Receiver<T> {
        let (new_sender, new_receiver) = mpsc::unbounded();
        self.new_senders.unbounded_send(new_sender).unwrap();
        new_receiver
    }

    pub fn send_item(&self, item: T) {
        self.new_messages.unbounded_send(item).unwrap();
    }
}
