use crate::models::user::UserForCreate;

/// This model is indented to be used for creating new user.
/// It provide only required data that should be provided by user itself.
#[derive(serde::Deserialize, validator::Validate)]
pub struct UserFromRequest {
  #[validate(length(min = 6))]
  username: String,

  #[validate(length(min = 8))]
  password: String,
}

impl TryInto<UserForCreate> for UserFromRequest {
  type Error = argon2::password_hash::Error;

  fn try_into(self) -> Result<UserForCreate, Self::Error> {
    Ok(UserForCreate::new(
      self.username,
      crate::security::hash(self.password)?
    ))
  }
}
