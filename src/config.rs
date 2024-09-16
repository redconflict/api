
#[derive(derive_more::From, derive_more::Display, derive_more::Debug)]
pub enum Error {
  #[allow(unused)]
  WrongValue,

  #[from(std::env::VarError)]
  MissingRequired
}

#[derive(Clone)]
pub struct Config {
  pub addr: String,
  pub db_dsn: String
}

impl Config {

  /// Create [`Config`] from process environment.
  pub fn from_env() -> Result<Self, Error> {
    Ok(Self {
      addr: std::env::var("APP_ADDR")?, 
      db_dsn: std::env::var("DB_DSN")?
    })
  }
}