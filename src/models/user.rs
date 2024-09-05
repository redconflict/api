
/// This model describe what user what a "user" is in database.
#[derive(serde::Serialize, Clone)]
pub struct User {
  #[serde(with = "uuid::serde::simple")]
  /* PRIMARY KEY */ pub id: uuid::Uuid,
  /* UNIQUE      */ pub username: String,
  // Password must never be serialized. 
  #[serde(skip)]
  pub password: String,
  /* DATETIME    */ pub created_at: u64,
  /* DATETIME    */ pub update_at: Option<u64>
}

/// This model is indented to be used for creating new user.
/// It provide only required data for creating new ones.
#[derive(serde::Deserialize)]
pub struct UserForCreate {
  pub username: String,
  pub password: String
}