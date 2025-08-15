use futures::{SinkExt, StreamExt};
use std::sync::Arc;

use axum::{
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use crdt::Rga;

use crate::{
    events::{RealtimeEvent, RealtimeEventKind},
    state::AppState,
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state, id))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, id: String) {
    let document = state
        .get_document(&id)
        .unwrap_or_else(|| state.insert_document(id, Rga::new(0)));
    let actor_id = document.new_actor();
    let (mut sender, mut receiver) = socket.split();

    let recv_doc = Arc::clone(&document);
    let recv_task = tokio::spawn(async move {
        let document = recv_doc;
        while let Some(msg) = receiver.next().await {
            let msg = if let Ok(msg) = msg {
                msg
            } else {
                return;
            };

            let event: RealtimeEventKind =
                serde_json::from_slice(msg.into_data().iter().as_slice()).unwrap();

            match &event {
                RealtimeEventKind::Insert {
                    id,
                    query,
                    contents,
                } => document.change(|state| {
                    state.insert(query.clone(), *contents, Some(id.0), Some(id.1));
                }),
            };

            document
                .sender()
                .send(RealtimeEvent {
                    actor: actor_id,
                    kind: event,
                })
                .unwrap();
        }
    });

    let send_task = tokio::spawn(async move {
        let mut recv = document.sender().subscribe();

        while let Ok(event) = recv.recv().await {
            if event.actor == actor_id {
                continue;
            }

            sender
                .send(Message::Text(
                    serde_json::to_string(&event.kind).unwrap().into(),
                ))
                .await
                .unwrap();
        }
    });

    tokio::select! {
      _ = recv_task => {},
      _ = send_task => {},
    }
}
