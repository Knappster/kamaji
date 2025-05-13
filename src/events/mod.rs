pub mod error;

use error::EventsError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result as ResultType};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Clone)]
struct Handler {
    handler: Arc<dyn Fn(&Event) + Send + Sync + 'static>,
    name: Option<String>,
}

impl Debug for Handler {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> ResultType {
        let mut debug_struct = fmt.debug_struct("Handler");
        if let Some(name) = &self.name {
            debug_struct.field("name", name);
        }
        debug_struct.finish_non_exhaustive()
    }
}

impl<F> From<F> for Handler
where
    F: Fn(&Event) + Send + Sync + 'static + 'static,
{
    fn from(f: F) -> Self {
        Handler {
            handler: Arc::new(f),
            name: None,
        }
    }
}

impl Handler {
    pub fn set_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn call(&self, event: &Event) {
        (self.handler)(event)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub event_type: String,
    pub payload: Value,
}

#[derive(Debug, Clone)]
struct EventHandler {
    id: String,
    handler: Handler,
}

#[derive(Debug, Clone)]
pub struct Events {
    broadcast: broadcast::Sender<Event>,
    handlers: HashMap<String, Vec<EventHandler>>,
}

impl Events {
    pub fn new() -> Result<Self, EventsError> {
        let (sender, _) = broadcast::channel::<Event>(1000);
        let mut receiver = sender.subscribe();

        tokio::spawn(async move {
            while let Ok(event) = receiver.recv().await {
                tracing::debug!(
                    "Event triggered: {} with payload: {:?}",
                    event.event_type,
                    event.payload
                );
            }
        });

        Ok(Self {
            broadcast: sender,
            handlers: HashMap::new(),
        })
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
            handler: Handler::from(handler),
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
                entry.handler.call(&event);
            }
        }
    }
}
