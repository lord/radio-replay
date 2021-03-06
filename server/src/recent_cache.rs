use async_std::prelude::StreamExt;
use futures::channel::mpsc::{self, UnboundedReceiver as Receiver, UnboundedSender as Sender};
use futures::future::FutureExt;
use futures::select;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct RecentCache<T: Send + Clone + 'static> {
    new_messages: Sender<T>,
    new_senders: Sender<Sender<T>>,
}

impl std::io::Write for RecentCache<Vec<u8>> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.send_item(buf.to_vec());
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// An async-channel cache. Allows sending new messages of type `T` to any number of listening Receivers.
/// When a new receiever connects with `get_stream`, it will receieve the most recent `capacity` messages
/// sent to the other streams. If capacity is `None`, the cache will have infinite size.
impl<T: Send + Clone + 'static> RecentCache<T> {
    pub fn new(capacity: Option<usize>) -> Self {
        let (new_messages_tx, new_messages_rx) = mpsc::unbounded();
        let (new_senders_tx, new_senders_rx) = mpsc::unbounded();
        async_std::task::spawn(Self::handler_task(
            capacity,
            new_messages_rx,
            new_senders_rx,
        ));
        Self {
            new_messages: new_messages_tx,
            new_senders: new_senders_tx,
        }
    }

    async fn handler_task(
        capacity: Option<usize>,
        mut new_messages: Receiver<T>,
        mut new_senders: Receiver<Sender<T>>,
    ) {
        let mut recent_messages = VecDeque::with_capacity(capacity.unwrap_or(0));
        let mut senders = Vec::new();
        let mut next_msg_fut = new_messages.next().fuse();
        let mut next_sender_fut = new_senders.next().fuse();
        loop {
            select! {
                next_msg = next_msg_fut => {
                    let item = match next_msg {
                        Some(v) => v,
                        None => continue,
                    };
                    if let Some(capacity) = capacity {
                        while recent_messages.len() >= capacity-1 && capacity > 0 {
                            recent_messages.pop_front();
                        }
                    }
                    if capacity != Some(0) {
                        recent_messages.push_back(item.clone());
                    }
                    senders.retain(|sender: &Sender<T>| {
                        sender.unbounded_send(item.clone()).is_ok()
                    });

                    next_msg_fut = new_messages.next().fuse();
                }
                next_sender = next_sender_fut => {
                    let sender = match next_sender {
                        Some(v) => v,
                        None => continue,
                    };
                    for message in recent_messages.iter() {
                        let _ = sender.unbounded_send(message.clone());
                    }
                    senders.push(sender);

                    next_sender_fut = new_senders.next().fuse();
                }
                complete => break,
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
