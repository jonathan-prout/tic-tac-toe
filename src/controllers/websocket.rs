use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use axum::routing::get;
use futures_util::{SinkExt, StreamExt};
use loco_rs::prelude::*;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::app::{GAME_STATE, WS_MANAGER};
use crate::websocket::WebSocketClient;

/// WebSocket endpoint for real-time game updates
pub async fn websocket_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    let client_id = Uuid::new_v4();
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    let client = WebSocketClient {
        id: client_id,
        sender: tx,
    };

    // Add client to manager
    WS_MANAGER.add_client(client).await;

    tracing::info!("WebSocket client {} connected", client_id);

    // Send initial game state
    let game_state = GAME_STATE.read().await;

    // Send all current tile states
    for x in 0..3 {
        for y in 0..3 {
            let tile_update = crate::websocket::TileUpdate::from(game_state.tiles[x][y]);
            let message = crate::websocket::WebSocketMessage {
                topic: format!("game.tile.{}.{}", x, y),
                payload: serde_json::to_value(tile_update).unwrap(),
            };

            if let Ok(json) = serde_json::to_string(&message) {
                if let Err(e) = sender.send(Message::Text(json)).await {
                    tracing::warn!("Failed to send initial tile state: {}", e);
                    break;
                }
            }
        }
    }

    // Send current game state
    let state_update = crate::websocket::StateUpdate::from(game_state.win_condition.clone());
    let message = crate::websocket::WebSocketMessage {
        topic: "game.state".to_string(),
        payload: serde_json::to_value(state_update).unwrap(),
    };

    if let Ok(json) = serde_json::to_string(&message) {
        if let Err(e) = sender.send(Message::Text(json)).await {
            tracing::warn!("Failed to send initial game state: {}", e);
        }
    }

    drop(game_state);

    // Spawn task to handle outgoing messages
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages (for potential future use)
    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    tracing::debug!("Received message from {}: {}", client_id, text);
                    // TODO: We'll extend this later to handle POST-style commands via WebSocket
                    // For now, just noop
                }
                Ok(Message::Close(_)) => {
                    tracing::info!("WebSocket client {} disconnected", client_id);
                    break;
                }
                Err(e) => {
                    tracing::warn!("WebSocket error for client {}: {}", client_id, e);
                    break;
                }
                _ => {
                    // Noop for other message types - we'll extend this later
                }
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }

    // Remove client from manager
    WS_MANAGER.remove_client(&client_id).await;
    tracing::info!("WebSocket client {} cleaned up", client_id);
}

pub fn routes() -> Routes {
    Routes::new().add("/ws", get(websocket_handler))
}
