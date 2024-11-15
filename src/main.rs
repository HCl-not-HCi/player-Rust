mod logic;
mod models;
use axum::{
    routing::{get, post},
    Json, Router,
};
use models::{game_state::GameState, player_action::PlayerAction};
use tracing::{debug, info};

const URL: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Start Rust player");
    debug!("Debugging turned on");
    debug!(?URL, "listening URL");

    let app = Router::new()
        .route("/", get(identify))
        .route("/", post(index));

    let listener = tokio::net::TcpListener::bind(URL).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn identify() -> String {
    let git_hash = env!("GIT_HASH");
    format!("Bitwars Rust Player (git_hash:\"{}\")", git_hash)
}

async fn index(Json(payload): Json<GameState>) -> Json<Vec<PlayerAction>> {
    Json(logic::strategy::decide(payload))
}
