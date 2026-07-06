use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
};
use chrono::Utc;
use futures_util::{StreamExt, stream};
use serde::Serialize;
use tokio_stream::wrappers::BroadcastStream;
use uuid::Uuid;

use crate::state::AppState;

pub const CATALOG_SSE_EVENT: &str = "catalog.changed";
const HEARTBEAT_SECS: u64 = 25;

#[derive(Clone, Serialize)]
pub struct CatalogChangedEvent {
    #[serde(rename = "type")]
    pub event_type: &'static str,
    pub resource: &'static str,
    pub action: &'static str,
    pub at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
}

pub struct CatalogEventHub {
    sender: tokio::sync::broadcast::Sender<String>,
}

impl CatalogEventHub {
    pub fn new() -> Arc<Self> {
        let (sender, _) = tokio::sync::broadcast::channel(256);
        Arc::new(Self { sender })
    }

    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<String> {
        self.sender.subscribe()
    }

    pub fn publish_product(&self, action: &'static str, product_id: Uuid, sku: &str) {
        let event = CatalogChangedEvent {
            event_type: CATALOG_SSE_EVENT,
            resource: "product",
            action,
            at: Utc::now().to_rfc3339(),
            id: Some(product_id),
            sku: Some(sku.to_owned()),
        };
        self.publish(event);
    }

    pub fn publish_category(&self, action: &'static str, category_id: Uuid) {
        let event = CatalogChangedEvent {
            event_type: CATALOG_SSE_EVENT,
            resource: "category",
            action,
            at: Utc::now().to_rfc3339(),
            id: Some(category_id),
            sku: None,
        };
        self.publish(event);
    }

    fn publish(&self, event: CatalogChangedEvent) {
        let Ok(payload) = serde_json::to_string(&event) else {
            return;
        };
        let _ = self.sender.send(payload);
    }
}

pub fn notify_product_changed(
    hub: &CatalogEventHub,
    action: &'static str,
    product_id: Uuid,
    sku: &str,
) {
    hub.publish_product(action, product_id, sku);
}

pub fn notify_category_changed(hub: &CatalogEventHub, action: &'static str, category_id: Uuid) {
    hub.publish_category(action, category_id);
}

pub async fn stream_catalog_events(
    State(state): State<AppState>,
) -> Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>> {
    let receiver = state.catalog_events.subscribe();
    let connected = stream::once(async {
        Ok(Event::default().comment("connected")) as Result<Event, Infallible>
    });
    let events = BroadcastStream::new(receiver).filter_map(|message| async move {
        match message {
            Ok(data) => Some(Ok(Event::default().event(CATALOG_SSE_EVENT).data(data))),
            Err(tokio_stream::wrappers::errors::BroadcastStreamRecvError::Lagged(_)) => None,
        }
    });
    let stream = connected.chain(events);

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(HEARTBEAT_SECS))
            .text("ping"),
    )
}
