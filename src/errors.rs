use crate::config;

#[derive(derive_more::From, derive_more::Display, derive_more::Debug)]
pub enum Error {
  #[from(config::Error)]
  #[display("config error: {_0}")]
  Config(config::Error),

  #[from(tokio::io::Error)]
  #[display("io error: {_0}")]
  Io(tokio::io::Error),

  #[from(argon2::password_hash::Error)]
  #[display("security error: {_0}")]
  Security(argon2::password_hash::Error),

  #[from(sqlx::Error)]
  #[display("db error: {_0}")]
  Db(sqlx::Error),
}
