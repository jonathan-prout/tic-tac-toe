use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

use crate::app::AppContext;
use crate::websocket::WebSocketClient;

/// WebSocket endpoint for real-time game updates
/// 
/// Connect to receive real-time updates about tile changes and game state
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(ctx): State<AppContext>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, ctx))
}

async fn handle_socket(socket: WebSocket, ctx: AppContext) {
    let client_id = Uuid::new_v4();
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    let client = WebSocketClient {
        id: client_id,
        sender: tx,
    };

    // Add client to manager
    ctx.ws_manager.add_client(client).await;

    tracing::info!("WebSocket client {} connected", client_id);

    // Send initial game state
    let game_state = ctx.game_state.read().await;
    
    // Send all current tile states
    for x in 0..3 {
        for y in 0..3 {
            let tile_update = crate::websocket::TileUpdate::from(game_state.tiles[x][y]);
            let message = crate::websocket::WebSocketMessage {
                topic: format!("game.tile.{}.{}", x, y),
                payload: serde_json::to_value(tile_update).unwrap(),
            };
            
            if let Ok(json) = serde_json::to_string(&message) {
                if let Err(e) = rx.try_recv() {
                    // Channel is empty, send the message
                    if let Err(e) = sender.send(Message::Text(json)).await {
                        tracing::warn!("Failed to send initial tile state: {}", e);
                    }
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
                    // Handle incoming messages if needed
                }
                Ok(Message::Close(_)) => {
                    tracing::info!("WebSocket client {} disconnected", client_id);
                    break;
                }
                Err(e) => {
                    tracing::warn!("WebSocket error for client {}: {}", client_id, e);
                    break;
                }
                _ => {}
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
    ctx.ws_manager.remove_client(&client_id).await;
    tracing::info!("WebSocket client {} cleaned up", client_id);
}

pub fn routes() -> Router<AppContext> {
    Router::new().route("/ws", get(websocket_handler))
}
