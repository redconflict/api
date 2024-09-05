use crate::config;

#[derive(derive_more::From, derive_more::Display, derive_more::Debug)]
pub enum Error {
  #[from(config::Error)]
  #[display("config error: {_0}")]
  Config(config::Error),

  #[from(tokio::io::Error)]
  #[display("io error: {_0}")]
  Io(tokio::io::Error),
}