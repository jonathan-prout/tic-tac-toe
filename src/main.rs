use loco_rs::prelude::*;
use tictactoe_server::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    App::new().run().await
}
