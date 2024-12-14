use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{pool, Pool, Postgres};
use uuid::Uuid;

use crate::models::{File, ReceivedFileDetails, SentFileDetails, SharedLink, User};

#[derive(Debug, Clone)]
pub struct DBClient{
  pool: Pool<Postgres>,
}

impl DBClient {
  pub fn new(pool: Pool<Postgres>) -> Self {
    DBClient {pool}
  }
}

#[async_trait]
pub trait UserExt {
  async fn get_user (
    &self,
    user_id: Option<Uuid>,
    name: Option<&str>,
    email: Option<&str>,

  ) -> Result<Option<User>, sqlx::Error>;
}

async fn saver_user<T: Into<String> + Send> (
  &self
)