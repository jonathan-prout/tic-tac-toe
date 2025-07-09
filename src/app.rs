use std::sync::Arc;
use loco_rs::prelude::*;
use axum::Router;
use tower_http::services::ServeDir;
use tokio::sync::RwLock;

use crate::{
    controllers,
    game::GameState,
    websocket::WebSocketManager,
};

pub struct App;

#[async_trait::async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        // Initialize game state
        let game_state = Arc::new(RwLock::new(GameState::new()));
        let ws_manager = Arc::new(WebSocketManager::new());

        Ok(BootResult {
            router: Some(Router::new()
                .nest_service("/static", ServeDir::new("static"))
                .with_state(AppContext {
                    game_state,
                    ws_manager,
                })),
            ..Default::default()
        })
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .add_route(controllers::game::routes())
            .add_route(controllers::websocket::routes())
    }

    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        // No background workers needed for this demo
        Ok(())
    }
}

#[derive(Clone)]
pub struct AppContext {
    pub game_state: Arc<RwLock<GameState>>,
    pub ws_manager: Arc<WebSocketManager>,
}

impl Default for AppContext {
    fn default() -> Self {
        Self {
            game_state: Arc::new(RwLock::new(GameState::new())),
            ws_manager: Arc::new(WebSocketManager::new()),
        }
    }
}
