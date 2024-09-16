/// Errors related to storage layer of this application
/// iex: Database, cache, et...
pub enum Error {
  /// Generic data store error.
  Generic(sqlx::Error),

  /// Unique constraint violation error.
  Duplicate(String),

  /// Expected a record, but data store returned nothing.
  NotFound
}

impl From<sqlx::Error> for Error {
  fn from(value: sqlx::Error) -> Self {
    match value {
      sqlx::Error::RowNotFound => Self::NotFound,
      sqlx::Error::Database(ref e) => {
        if let Some(kc) = e.constraint() {
          if kc.to_lowercase().contains("unique") {
            return Self::Duplicate(kc.to_string())
          }
        }

        Self::Generic(value)
      }
      _ => Self::Generic(value)
    }
  }
}

pub type Result<T> = std::result::Result<T, Error>;

/// Implements [`CRUD<T, C>`] trait on models to enable CRUD operations
/// on data store.
pub trait Crud<T, C> {
  async fn delete(&self, entity: &mut T) -> Result<()>;

  async fn save(&self, entity: &T) -> Result<()>;

  async fn create(&self, dto: C) -> Result<T>;

  async fn all(&self) -> Result<Vec<T>>;

  async fn with_id(&self, id: uuid::Uuid) -> Result<T>;
}
