use std::sync::Arc;

use axum::{
  extract::State, http::StatusCode, routing::get, Json, Router
};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
  models::{User, UserForCreate},
  state::AppState,
};

pub fn router() -> Router<Arc<Mutex<AppState>>> {
  // State (context) shared between all endpoint/middlewares.
  Router::new()
    .route("/", get(get_users).post(create_user))
}

#[derive(serde::Serialize)]
struct Users {
  users: Vec<User>
}

async fn get_users(State(state): State<Arc<Mutex<AppState>>>) -> Json<Users> {
  let clone = state.lock().await.clone();
  Json(Users { users: clone.users })
}

async fn create_user(
  State(state): State<Arc<Mutex<AppState>>>,
  Json(payload): Json<UserForCreate>,
) -> (StatusCode, Json<User>) {

  // Creating new user.
  let user = User {
    id: Uuid::new_v4(),
    created_at: chrono::Local::now().timestamp() as u64,
    update_at: None,
    username: payload.username,
    password: payload.password,
  };

  // Saving to state.
  state.lock().await.add_user(user.clone());

  // Return user.
  (StatusCode::CREATED, Json(user))
}
