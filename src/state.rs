use crate::models::User;

#[derive(Clone)]
pub struct AppState {
  pub users: Vec<User>,
}

impl AppState {
  pub fn new() -> Self {
    AppState { users: Vec::new() }
  }

  pub fn add_user(&mut self, user: User) {
    self.users.push(user)
  }
}