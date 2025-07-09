use axum::extract::Path;
use axum::response::{IntoResponse, Json};
use axum::routing::{get, post};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::app::{GAME_STATE, WS_MANAGER};
use crate::game::{TileState, WinCondition};
use crate::websocket::{StateUpdate, TileUpdate};

// Explicit import of Response type
use axum::response::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct TileGrid {
    pub tiles: [[TileState; 3]; 3],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameStateResponse {
    pub condition: WinCondition,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveRequest {
    pub state: TileState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Get current tile grid
pub async fn get_tiles() -> Result<Response> {
    let game_state = GAME_STATE.read().await;
    let response = TileGrid {
        tiles: game_state.tiles,
    };
    Ok(Json(response).into_response())
}

/// Get current game state
pub async fn get_state() -> Result<Response> {
    let game_state = GAME_STATE.read().await;
    let response = GameStateResponse {
        condition: game_state.win_condition.clone(),
    };
    Ok(Json(response).into_response())
}

/// Make a move
pub async fn make_move(
    Path((x, y)): Path<(usize, usize)>,
    Json(move_request): Json<MoveRequest>,
) -> Result<Response> {
    let mut game_state = GAME_STATE.write().await;
    let old_condition = game_state.win_condition.clone();

    match game_state.make_move(x, y, move_request.state) {
        Ok(()) => {
            // Publish tile update
            let tile_update = TileUpdate::from(move_request.state);
            WS_MANAGER
                .publish(
                    &format!("game.tile.{}.{}", x, y),
                    serde_json::to_value(tile_update).unwrap(),
                )
                .await;

            // Publish state update if win condition changed
            if game_state.win_condition != old_condition {
                let state_update = StateUpdate::from(game_state.win_condition.clone());
                WS_MANAGER
                    .publish("game.state", serde_json::to_value(state_update).unwrap())
                    .await;
            }

            Ok(().into_response())
        }
        Err("Tile already occupied") => Err(Error::BadRequest("Tile already occupied".into())),
        Err(msg) => Err(Error::BadRequest(msg.into())),
    }
}

/// Reset game
pub async fn reset_game() -> Result<Response> {
    let mut game_state = GAME_STATE.write().await;
    game_state.reset();

    // Publish reset events for all tiles
    for x in 0..3 {
        for y in 0..3 {
            let tile_update = TileUpdate::from(TileState::Empty);
            WS_MANAGER
                .publish(
                    &format!("game.tile.{}.{}", x, y),
                    serde_json::to_value(tile_update).unwrap(),
                )
                .await;
        }
    }

    // Publish state reset
    let state_update = StateUpdate::from(WinCondition::NoWin);
    WS_MANAGER
        .publish("game.state", serde_json::to_value(state_update).unwrap())
        .await;

    Ok(().into_response())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api")
        .add("/tiles", get(get_tiles))
        .add("/state", get(get_state))
        .add("/tile/:x/:y", post(make_move))
        .add("/reset", post(reset_game))
}
