use std::sync::Arc;

use crate::models::Crud;
use axum::response::ErrorResponse;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use axum_valid::Valid;
use tokio::sync::Mutex;

use crate::models::user::{User, Users};
use crate::state::AppState;
use crate::web::payloads::UserFromRequest;

pub fn router() -> Router<Arc<Mutex<AppState>>> {
  // State (context) shared between all endpoint/middlewares.
  Router::new().route("/", get(get_users).post(create_user))
}

async fn get_users(
  State(state): State<Arc<Mutex<AppState>>>,
) -> axum::response::Result<Json<Vec<User>>> {
  let st = state.lock().await;
  if let Ok(users) = Users::with(&st.pool).all().await {
    return Ok(Json(users));
  }

  Err(ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))
}

async fn create_user(
  State(state): State<Arc<Mutex<AppState>>>,
  Valid(Json(payload)): Valid<Json<UserFromRequest>>,
) -> axum::response::Result<Json<User>> {
  let st = state.lock().await;
  if let Ok(ufc) = payload.try_into() {
    if let Ok(user) = Users::with(&st.pool).create(ufc).await {
      return Ok(Json(user))
    }
  }

  Err(ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))
}
