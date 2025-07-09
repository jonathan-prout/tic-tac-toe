use lazy_static::lazy_static;
use loco_rs::app::Hooks;
use loco_rs::boot::create_app;
use loco_rs::boot::BootResult;
use loco_rs::boot::StartMode;
use loco_rs::controller::AppRoutes;
use loco_rs::environment::Environment;
use loco_rs::prelude::*;
use loco_rs::task::Tasks;
use loco_rs::worker::Processor;
use sea_orm::DatabaseConnection;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{controllers, game::GameState, websocket::WebSocketManager};

pub struct App;

// Global state that we want to share - make them public
lazy_static! {
    pub static ref GAME_STATE: Arc<RwLock<GameState>> = Arc::new(RwLock::new(GameState::new()));
    pub static ref WS_MANAGER: Arc<WebSocketManager> = Arc::new(WebSocketManager::new());
}

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
        // Use the with-migrator boot method with empty migrator
        create_app::<Self, migration::Migrator>(mode, environment).await
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .add_route(controllers::game::routes())
            .add_route(controllers::websocket::routes())
            .add_route(controllers::static_files::routes())
    }

    fn connect_workers<'a>(_p: &'a mut Processor, _ctx: &'a AppContext) {
        // No workers needed for this demo
    }

    fn register_tasks(_tasks: &mut Tasks) {
        // No tasks needed for this demo
    }

    async fn truncate(_db: &DatabaseConnection) -> Result<()> {
        // No tables to truncate for in-memory game
        Ok(())
    }

    async fn seed(_db: &DatabaseConnection, _base: &Path) -> Result<()> {
        // No seeding needed for in-memory game
        Ok(())
    }
}
