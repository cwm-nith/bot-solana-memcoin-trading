use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, prelude::*};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
  pub id: i32,
  pub token: String,
  pub is_sold: bool,
  pub buy_at_price: f32,
  pub sold_at_price: f32,
  pub balance: f32,
  pub fee: f32,
  pub metadata: String,
  pub created_at: Option<DateTime<Utc>>,
  pub updated_at: Option<DateTime<Utc>>,
}

impl<'r> FromRow<'r, PgRow> for TokenData {
  fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
    Ok(TokenData {
      id: row.try_get("id")?,
      token: row.try_get("token")?,
      is_sold: row.try_get("is_sold")?,
      buy_at_price: row.try_get("buy_at_price")?,
      sold_at_price: row.try_get("sold_at_price")?,
      balance: row.try_get("balance")?,
      fee: row.try_get("fee")?,
      metadata: row.try_get("metadata")?,
      created_at: row.try_get("created_at")?,
      updated_at: row.try_get("updated_at")?,
    })
  }
}
