use std::sync::Arc;

use crate::data::Crud;
use axum::response::ErrorResponse;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use tokio::sync::Mutex;

use crate::models::UserFromRequest;
use crate::{
  models::User,
  state::AppState,
};

pub fn router() -> Router<Arc<Mutex<AppState>>> {
  // State (context) shared between all endpoint/middlewares.
  Router::new().route("/", get(get_users).post(create_user))
}

async fn get_users(State(state): State<Arc<Mutex<AppState>>>) -> axum::response::Result<Json<Vec<User>>> {
  let st = state.lock().await;
  if let Ok(users) = User::all(st.store()).await {
    return Ok(Json(users))
  }

  Err(ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))
}

async fn create_user(
  State(state): State<Arc<Mutex<AppState>>>,
  Json(payload): Json<UserFromRequest>,
) -> axum::response::Result<Json<User>> {
  if let Ok(u) = payload.for_create() {
    // Creating new user.
    let st = state.lock().await;
    if let Ok(user) = User::create(st.store(), u).await {
      return Ok(Json(user))
    }
  }

  Err(ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))
}
