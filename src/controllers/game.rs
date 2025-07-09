use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use axum::routing::{get, post};
use axum::Router;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::app::AppContext;
use crate::game::{TileState, WinCondition};
use crate::websocket::{TileUpdate, StateUpdate};

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
/// 
/// Returns the current 3x3 grid of tile states
#[utoipa::path(
    get,
    path = "/api/tiles",
    responses(
        (status = 200, description = "Current tile grid", body = TileGrid)
    )
)]
pub async fn get_tiles(State(ctx): State<AppContext>) -> Json<TileGrid> {
    let game_state = ctx.game_state.read().await;
    Json(TileGrid {
        tiles: game_state.tiles,
    })
}

/// Get current game state
/// 
/// Returns the current win condition of the game
#[utoipa::path(
    get,
    path = "/api/state",
    responses(
        (status = 200, description = "Current game state", body = GameStateResponse)
    )
)]
pub async fn get_state(State(ctx): State<AppContext>) -> Json<GameStateResponse> {
    let game_state = ctx.game_state.read().await;
    Json(GameStateResponse {
        condition: game_state.win_condition.clone(),
    })
}

/// Make a move
/// 
/// Attempt to place X or O at the specified coordinates
#[utoipa::path(
    post,
    path = "/api/tile/{x}/{y}",
    params(
        ("x" = usize, Path, description = "X coordinate (0-2)"),
        ("y" = usize, Path, description = "Y coordinate (0-2)")
    ),
    request_body = MoveRequest,
    responses(
        (status = 200, description = "Move successful"),
        (status = 409, description = "Tile already occupied", body = ErrorResponse),
        (status = 400, description = "Invalid coordinates", body = ErrorResponse)
    )
)]
pub async fn make_move(
    State(ctx): State<AppContext>,
    Path((x, y)): Path<(usize, usize)>,
    Json(move_request): Json<MoveRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let mut game_state = ctx.game_state.write().await;
    let old_condition = game_state.win_condition.clone();
    
    match game_state.make_move(x, y, move_request.state) {
        Ok(()) => {
            // Publish tile update
            let tile_update = TileUpdate::from(move_request.state);
            ctx.ws_manager
                .publish(
                    &format!("game.tile.{}.{}", x, y),
                    serde_json::to_value(tile_update).unwrap(),
                )
                .await;

            // Publish state update if win condition changed
            if game_state.win_condition != old_condition {
                let state_update = StateUpdate::from(game_state.win_condition.clone());
                ctx.ws_manager
                    .publish(
                        "game.state",
                        serde_json::to_value(state_update).unwrap(),
                    )
                    .await;
            }

            Ok(StatusCode::OK)
        }
        Err("Tile already occupied") => Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "Tile already occupied".to_string(),
            }),
        )),
        Err(msg) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: msg.to_string(),
            }),
        )),
    }
}

/// Reset game
/// 
/// Clear the game board and reset win condition
#[utoipa::path(
    post,
    path = "/api/reset",
    responses(
        (status = 200, description = "Game reset successful")
    )
)]
pub async fn reset_game(State(ctx): State<AppContext>) -> StatusCode {
    let mut game_state = ctx.game_state.write().await;
    game_state.reset();

    // Publish reset events for all tiles
    for x in 0..3 {
        for y in 0..3 {
            let tile_update = TileUpdate::from(TileState::Empty);
            ctx.ws_manager
                .publish(
                    &format!("game.tile.{}.{}", x, y),
                    serde_json::to_value(tile_update).unwrap(),
                )
                .await;
        }
    }

    // Publish state reset
    let state_update = StateUpdate::from(WinCondition::NoWin);
    ctx.ws_manager
        .publish("game.state", serde_json::to_value(state_update).unwrap())
        .await;

    StatusCode::OK
}

pub fn routes() -> Router<AppContext> {
    Router::new()
        .route("/api/tiles", get(get_tiles))
        .route("/api/state", get(get_state))
        .route("/api/tile/:x/:y", post(make_move))
        .route("/api/reset", post(reset_game))
}
