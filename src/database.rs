use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use thiserror::Error;

use crate::model::TokenRecord;

#[derive(Error, Debug)]
pub enum DatabaseError {
  #[error("Database connection error: {0}")]
  ConnectionError(String),
  #[error("Query execution error: {0}")]
  QueryError(String),
}
#[derive(Debug, Clone)]
pub struct Database {
  pool: SqlitePool,
}

impl Database {
  pub async fn new(db_url: &str) -> Result<Self, DatabaseError> {
    let pool = SqlitePoolOptions::new()
      .connect(db_url)
      .await
      .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

    sqlx::migrate!()
      .run(&pool)
      .await
      .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

    Ok(Self { pool })
  }

  pub async fn _save_transaction(&self, record: &TokenRecord) -> Result<(), DatabaseError> {
    sqlx::query!(
      r#"
            INSERT INTO tokens 
            (mint_address, balance, entry_price, fees, metadata, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
      record.mint_address,
      record.balance,
      record.entry_price,
      record.fees,
      record.metadata,
      record.timestamp
    )
    .execute(&self.pool)
    .await
    .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

    Ok(())
  }

  pub async fn get_all_tokens(&self) -> Result<Vec<TokenRecord>, DatabaseError> {
    sqlx::query_as!(TokenRecord, r#"SELECT * FROM tokens"#)
      .fetch_all(&self.pool)
      .await
      .map_err(|e| DatabaseError::QueryError(e.to_string()))
  }
}
