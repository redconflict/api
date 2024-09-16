mod models;
mod web;
mod state;
mod config;
mod errors;
mod security;

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), errors::Error> {
  // Load config
  let cfg = config::Config::from_env()?;

  // Initialize AppState 
  let pool = sqlx::postgres::PgPoolOptions::new()
    .max_connections(5)
    .connect(&cfg.db_dsn).await?;
  
  let state = Arc::new(Mutex::new(state::AppState::new(pool)));

  // Initialize web server.
  println!("Running server on {}", cfg.addr);
  let listener = tokio::net::TcpListener::bind(cfg.addr).await?;
  
  let router = axum::Router::new()
    .nest("/users", web::routes::user::router())
    .with_state(state);

  // Listen to HTTP request.
  axum::serve(listener, router).await?;

  Ok(())
}
