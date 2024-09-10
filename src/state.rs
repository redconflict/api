use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
  pub pool: Pool<Postgres>
}

impl AppState {
  pub fn new(pool: Pool<Postgres>) -> Self {
    AppState { pool }
  }

  pub fn store(&self) -> &Pool<Postgres> {
    &self.pool
  }
}