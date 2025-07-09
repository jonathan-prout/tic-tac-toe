use loco_rs::cli;
use tictactoe_server::app::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    cli::main::<App, migration::Migrator>().await
}
