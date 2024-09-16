use chrono::{DateTime, Local};

/// This model describe what user what a "user" is in database.
#[derive(serde::Serialize, Clone, sqlx::FromRow)]
pub struct User {
  /// Entity unique id, accessible throught `User::id()` function.
  #[serde(with = "uuid::serde::simple")]
  #[sqlx(rename = "public_id")]
  pub(in crate::models::user) id: uuid::Uuid,

  // Usersame, can be modified at a anytime.
  pub(in crate::models) username: String,

  // Password mustn't be serialized.
  // Prevent against API critical data leaks.
  // To update user's password `User::update_passwd(String)` function.
  #[serde(skip)]
  pub(in crate::models) passwd_hash: String,

  ////////////////
  // Timestamps //
  ////////////////

  // This timestamp is automatically generated when
  // user is inserted in database.
  pub(in crate::models) created_at: DateTime<Local>,

  // This timestamp is automatically generated when
  // any modification is done on this record in database.
  pub(in crate::models) updated_at: Option<DateTime<Local>>,

  // This timestamp is defined when user is deleted from database.
  // SOFT-DELETE.
  pub(in crate::models) deleted_at: Option<DateTime<Local>>,
}

impl User {
  pub fn id(&self) -> uuid::Uuid {
    self.id
  }

  /// Re-hash the given new password and update current user.
  pub fn update_passwd(&mut self, new: String) {
    // Encrypt new password here.
    self.passwd_hash = new;
  }

  pub fn is_deleted(&self) -> bool {
    self.deleted_at.is_some()
  }

  pub fn created_at(&self) -> DateTime<Local> {
    self.created_at
  }

  pub fn updated_at(&self) -> Option<DateTime<Local>> {
    self.updated_at
  }
}
