use std::sync::Arc;

use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use crdt::Rga;

use crate::{events::RealtimeEvent, state::AppState};

pub async fn handler(State(_): State<Arc<AppState>>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    let mut rga = Rga::new(0);

    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            return;
        };

        let event: RealtimeEvent =
            serde_json::from_slice(msg.into_data().iter().as_slice()).unwrap();

        match event {
            RealtimeEvent::Insert {
                id,
                query,
                contents,
            } => rga.insert(query, contents, Some(id.0), Some(id.1)),
        };

        socket
            .send(Message::Text(rga.to_string().into()))
            .await
            .unwrap();
    }
}
