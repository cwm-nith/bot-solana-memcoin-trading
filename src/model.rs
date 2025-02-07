use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct TokenData {
//   pub id: i32,
//   pub token: String,
//   pub is_sold: bool,
//   pub buy_at_price: f32,
//   pub sold_at_price: f32,
//   pub balance: f32,
//   pub fee: f32,
//   pub metadata: String,
//   pub created_at: Option<DateTime<Utc>>,
//   pub updated_at: Option<DateTime<Utc>>,
// }

#[derive(sqlx::FromRow, Debug)]
pub struct TokenRecord {
  pub id: i64,
  pub mint_address: String,
  pub balance: f64,
  pub entry_price: f64,
  pub fees: f64,
  pub metadata: String,
  pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamMessage {
  pub jsonrpc: Option<String>,
  pub method: Option<String>,
  pub params: Option<Params>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Params {
  pub result: Option<Result>,
  pub subscription: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Result {
  pub context: Option<Context>,
  pub value: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Context {
  pub slot: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
  pub signature: Option<String>,
  pub logs: Option<Vec<String>>,
}
