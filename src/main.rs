mod models;
mod routes;
mod state;
mod config;
mod errors;

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), errors::Error> {
  // Load config
  let cfg = config::Config::from_env()?;
  println!("Running server on {}", cfg.addr);
  let listener = tokio::net::TcpListener::bind(cfg.addr).await?;

  let state = Arc::new(Mutex::new(state::AppState::new()));
  let router = axum::Router::new()
    .nest("/users", routes::user::router())
    .with_state(state);

  axum::serve(listener, router).await?;

  Ok(())
}
