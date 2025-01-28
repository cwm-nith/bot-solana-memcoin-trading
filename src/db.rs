use sqlx::{Error, Pool, Postgres};

use crate::model::TokenData;

#[derive(Debug, Clone)]
pub struct DbClient {
  pub pool: Pool<Postgres>,
}

impl DbClient {
  pub fn new(pool: Pool<Postgres>) -> Self {
    Self { pool }
  }
}

pub trait DbRepo {
  async fn insert_token(&self, new_token: TokenData) -> Result<(), Error>;
}

impl DbRepo for DbClient {
  async fn insert_token(&self, new_token: TokenData) -> Result<(), Error> {
    let token = sqlx::query_as!(
      TokenData,
      r#"INSERT INTO tokens (
            token,
            is_sold,
            buy_at_price,
            sold_at_price,
            balance,
            fee,
            metadata,
            created_at,
            updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
      new_token.token,
      new_token.is_sold,
      new_token.buy_at_price,
      new_token.sold_at_price,
      new_token.balance,
      new_token.fee,
      new_token.metadata,
      new_token.created_at,
      new_token.updated_at
    )
    .fetch_one(&self.pool)
    .await?;
    Ok(())
  }
}
