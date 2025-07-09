use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::tungstenite::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub topic: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct WebSocketClient {
    pub id: Uuid,
    pub sender: mpsc::UnboundedSender<Message>,
}

pub struct WebSocketManager {
    clients: Arc<RwLock<HashMap<Uuid, WebSocketClient>>>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_client(&self, client: WebSocketClient) {
        let mut clients = self.clients.write().await;
        clients.insert(client.id, client);
    }

    pub async fn remove_client(&self, client_id: &Uuid) {
        let mut clients = self.clients.write().await;
        clients.remove(client_id);
    }

    pub async fn publish(&self, topic: &str, payload: serde_json::Value) {
        let message = WebSocketMessage {
            topic: topic.to_string(),
            payload,
        };

        let message_json = match serde_json::to_string(&message) {
            Ok(json) => json,
            Err(e) => {
                tracing::error!("Failed to serialize websocket message: {}", e);
                return;
            }
        };

        let clients = self.clients.read().await;
        for client in clients.values() {
            if let Err(e) = client.sender.send(Message::Text(message_json.clone())) {
                tracing::warn!("Failed to send message to client {}: {}", client.id, e);
            }
        }
    }

    pub async fn client_count(&self) -> usize {
        let clients = self.clients.read().await;
        clients.len()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileUpdate {
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateUpdate {
    pub condition: String,
}

impl From<crate::game::TileState> for TileUpdate {
    fn from(state: crate::game::TileState) -> Self {
        Self {
            state: match state {
                crate::game::TileState::Empty => "Empty".to_string(),
                crate::game::TileState::X => "X".to_string(),
                crate::game::TileState::O => "O".to_string(),
            },
        }
    }
}

impl From<crate::game::WinCondition> for StateUpdate {
    fn from(condition: crate::game::WinCondition) -> Self {
        Self {
            condition: match condition {
                crate::game::WinCondition::NoWin => "NoWin".to_string(),
                crate::game::WinCondition::XWin => "XWin".to_string(),
                crate::game::WinCondition::OWin => "OWin".to_string(),
                crate::game::WinCondition::Stalemate => "Stalemate".to_string(),
            },
        }
    }
}
