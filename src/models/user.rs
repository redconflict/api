use chrono::{DateTime, Local};
use sqlx::{postgres::PgExecutor, query_as, Executor, FromRow, Row};

use argon2::{
  password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
  Argon2,
};

use crate::{config::Error, data::Crud};

/// This model describe what user what a "user" is in database.
#[derive(serde::Serialize, Clone, sqlx::FromRow)]
pub struct User {
  #[serde(with = "uuid::serde::simple")]
  #[sqlx(rename = "public_id")]
  id: uuid::Uuid,

  pub username: String,

  // Password mustn't be serialized.
  // Prevent against API critical data leaks.
  #[serde(skip)]
  passwd_hash: String,

  created_at: DateTime<Local>,
  updated_at: Option<DateTime<Local>>,
  deleted_at: Option<DateTime<Local>>,
}

impl User {
  pub fn id(&self) -> uuid::Uuid {
    self.id
  }

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

/// This model is indented to be used for creating new user.
/// It provide only required data that should be provided by user itself.
#[derive(serde::Deserialize)]
pub struct UserFromRequest {
  pub username: String,
  pub password: String,
}

impl UserFromRequest {
  pub fn for_create(self) -> Result<UserForCreate, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
      .hash_password(self.password.as_bytes(), &salt)?
      .to_string();

    Ok(UserForCreate {
      username: self.username,
      password: password_hash,
    })
  }
}

pub struct UserForCreate {
  pub username: String,
  pub password: String,
}

/// Database CRUD operations implementation.
impl Crud<User, UserForCreate> for User {
  type Db = sqlx::Pool<sqlx::Postgres>;

  async fn delete(&mut self, db: &Self::Db) -> crate::data::Result<()> {
    let ret: (DateTime<Local>,) = sqlx::query_as(
      "
      UPDATE RC.USERS
        SET
          RC.USERS.DELETED_AT = now()
        WHERE RC.USERS.PUBLIC_ID = $1
      RETURNING RC.USERS.DELETED_AT;
    ",
    )
    .bind(self.id.to_string())
    .fetch_one(db)
    .await?;

    self.deleted_at = Some(ret.0);

    Ok(())
  }

  async fn save(&self, db: &Self::Db) -> crate::data::Result<()> {
    sqlx::query(
      "
      UPDATE RC.USERS
        SET
          RC.USERS.USERNAME = $1,
          RC.USERS.PASSWD_HASH = $2
        WHERE RC.USERS.PUBLIC_ID = $3
    ",
    )
    .bind(&self.username)
    .bind(&self.passwd_hash)
    .bind(self.id.to_string())
    .execute(db)
    .await?;

    Ok(())
  }

  async fn create(db: &Self::Db, dto: UserForCreate) -> crate::data::Result<User> {
    let user = sqlx::query_as(
      "
      INSERT INTO RC.USERS (USERNAME, PASSWD_HASH)
        VALUES($1, $2)
      RETURNING
        RC.USERS.PUBLIC_ID,
        RC.USERS.USERNAME,
        RC.USERS.PASSWD_HASH,
        RC.USERS.UPDATED_AT,
        RC.USERS.CREATED_AT,
        RC.USERS.DELETED_AT;
    ",
    )
    .bind(dto.username)
    .bind(dto.password)
    .fetch_one(db)
    .await;

    match user {
      Ok(u) => Ok(u),
      Err(e) => {
        println!("{e}");
        Err(crate::data::Error::Sql(e))
      }
    }
  }

  async fn all(db: &Self::Db) -> crate::data::Result<Vec<User>> {
    let users = sqlx::query_as(
      "
      SELECT
        RC.USERS.PUBLIC_ID,
        RC.USERS.USERNAME,
        RC.USERS.PASSWD_HASH,
        RC.USERS.UPDATED_AT,
        RC.USERS.CREATED_AT,
        RC.USERS.DELETED_AT 
      FROM RC.USERS;
    ",
    )
    .fetch_all(db)
    .await?;

    Ok(users)
  }

  async fn with_id(db: &Self::Db, id: uuid::Uuid) -> crate::data::Result<User> {
    let user: User = sqlx::query_as(
      "
      SELECT
        RC.USERS.PUBLIC_ID,
        RC.USERS.USERNAME,
        RC.USERS.PASSWD_HASH,
        RC.USERS.UPDATED_AT,
        RC.USERS.CREATED_AT,
        RC.USERS.DELETED_AT
      FROM RC.USERS
      WHERE RC.USERS.PUBLIC_ID = $1;
    ",
    )
    .bind(id.to_string())
    .fetch_one(db)
    .await?;

    Ok(user)
  }
}
