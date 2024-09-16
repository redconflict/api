use chrono::{DateTime, Local};
use sqlx::{Pool, Postgres};

use crate::models::Crud;

use super::User;

pub struct UserForCreate {
  username: String,
  passwd_hash: String,
}

impl UserForCreate {
  pub fn new(username: String, passwd_hash: String) -> Self {
    Self { username, passwd_hash }
  }
}

pub struct Users<'s>(&'s Pool<Postgres>);

impl<'d> Users<'d> {
  /// Constructing new user database instance.
  pub fn with(db: &'d Pool<Postgres>) -> Self {
    Self(db)
  }
}

impl<'d> Crud<User, UserForCreate> for Users<'d> {
  async fn delete(&self, entity: &mut User) -> crate::models::crud::Result<()> {
    let (deletion_dt,): (DateTime<Local>,) = sqlx::query_as(
      "
      UPDATE RC.USERS
        SET
          RC.USERS.DELETED_AT = now()
        WHERE RC.USERS.PUBLIC_ID = $1
      RETURNING RC.USERS.DELETED_AT;
    ",
    )
    .bind(entity.id().to_string())
    .fetch_one(self.0)
    .await?;

    entity.deleted_at = Some(deletion_dt);

    Ok(())
  }

  async fn save(&self, entity: &User) -> crate::models::crud::Result<()> {
    sqlx::query(
      "
      UPDATE RC.USERS
        SET
          RC.USERS.USERNAME = $1,
          RC.USERS.PASSWD_HASH = $2
        WHERE RC.USERS.PUBLIC_ID = $3
    ",
    )
    .bind(&entity.username)
    .bind(&entity.passwd_hash)
    .bind(entity.id)
    .execute(self.0)
    .await?;

    Ok(())
  }

  async fn create(&self, dto: UserForCreate) -> crate::models::crud::Result<User> {
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
    .bind(dto.passwd_hash)
    .fetch_one(self.0)
    .await?;

    Ok(user)
  }

  async fn all(&self) -> crate::models::crud::Result<Vec<User>> {
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
    .fetch_all(self.0)
    .await?;

    Ok(users)
  }

  async fn with_id(&self, id: uuid::Uuid) -> crate::models::crud::Result<User> {
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
    .fetch_one(self.0)
    .await?;

    Ok(user)
  }
}
