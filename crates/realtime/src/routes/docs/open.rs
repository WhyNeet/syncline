use crdt_store::RgaSerializer;
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

    sender
        .send(Message::text(
            serde_json::to_string(&serde_json::json!({ "actor_id": actor_id })).unwrap(),
        ))
        .await
        .unwrap();

    sender
        .send(Message::text(
            serde_json::to_string(&RealtimeEvent {
                actor: 0,
                kind: RealtimeEventKind::StateSync {
                    state: document.change(|state| RgaSerializer::to_vec(state)),
                },
                version: document.version(),
            })
            .unwrap(),
        ))
        .await
        .unwrap();
    let (shutdown_tx, _) = tokio::sync::broadcast::channel::<()>(1);

    let mut shutdown = shutdown_tx.subscribe();
    let recv_doc = Arc::clone(&document);
    let recv_task = tokio::spawn(async move {
        let document = recv_doc;
        loop {
            tokio::select! {
              msg = receiver.next() => {
                  let msg = if let Some(Ok(msg)) = msg {
                      msg
                  } else {
                      continue;
                  };

                  if let Message::Close(_) = msg {
                      break;
                  }

                  let mut event: RealtimeEvent =
                      serde_json::from_slice(msg.into_data().iter().as_slice()).unwrap();

                  match &mut event.kind {
                      RealtimeEventKind::Insert {
                          id,
                          query,
                          contents,
                      } => document.change(|state| {
                          id.0 = actor_id;
                          state.insert(query.clone(), *contents, Some(actor_id), Some(id.1));
                      }),
                      RealtimeEventKind::Delete { id } => document.change(|state| {
                          state.delete(*id);
                      }),
                      _ => continue,
                  };

                  event.version = document.version();

                  document
                      .sender()
                      .send(event)
                      .unwrap();
              }
              _ = shutdown.recv() => break
            }
        }
    });

    let mut shutdown = shutdown_tx.subscribe();
    let send_doc = Arc::clone(&document);
    let mut compaction_rx = document.on_compaction();
    let send_task = tokio::spawn(async move {
        let document = send_doc;
        let mut recv = document.sender().subscribe();

        loop {
            tokio::select! {
              event = recv.recv() => {
                  let Ok(event) = event else {
                    break;
                  };
                  if event.actor == actor_id {
                      continue;
                  }

                  sender
                      .send(Message::Text(
                          serde_json::to_string(&event).unwrap().into(),
                      ))
                      .await
                      .unwrap();
              },
              _ = compaction_rx.recv() => {
                sender.send(Message::Text(serde_json::to_string(&RealtimeEvent {
                  kind: RealtimeEventKind::Compact,
                  version: document.version(),
                  actor: 0
                }).unwrap().into())).await.unwrap();
              },
              _ = shutdown.recv() => break
            }
        }
    });

    tokio::select! {
      _ = recv_task => {
        document.remove_actor(actor_id);
        shutdown_tx.send(()).unwrap();
      },
      _ = send_task => {
        document.remove_actor(actor_id);
        shutdown_tx.send(()).unwrap();
      },
    }
}
