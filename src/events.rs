use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub event_type: String,
    pub payload: Value,
}

struct EventHandler {
    id: String,
    handler: Box<dyn Fn(&Event) + Send + Sync + 'static>,
}

pub struct Events {
    broadcast: broadcast::Sender<Event>,
    handlers: HashMap<String, Vec<EventHandler>>,
}

impl Events {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel::<Event>(1000);

        Self {
            broadcast: sender,
            handlers: HashMap::new(),
        }
    }

    pub fn subscribe<F>(&mut self, event_type: &str, handler: F) -> String
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        let handlers = self
            .handlers
            .entry(event_type.to_string())
            .or_insert_with(Vec::new);

        let id = Uuid::new_v4().to_string();

        handlers.push(EventHandler {
            id: id.clone(),
            handler: Box::new(handler),
        });

        id
    }

    pub fn subscribe_all(&self) -> broadcast::Receiver<Event> {
        self.broadcast.subscribe()
    }

    pub fn unsubscribe_all(_receiver: broadcast::Receiver<Event>) {
        // Simply drop the receiver.
    }

    pub async fn publish(&self, event: Event) {
        // First, broadcast to channel subscribers.
        let _ = self.broadcast.send(event.clone());

        // Then, invoke direct handlers.
        if let Some(handlers) = self.handlers.get(&event.event_type) {
            for entry in handlers {
                (entry.handler)(&event);
            }
        }
    }
}
