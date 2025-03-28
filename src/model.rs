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

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
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

pub type TrxDetailRes = Vec<TrxDetailRe>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrxDetailRe {
  pub description: String,

  #[serde(rename = "type")]
  pub trx_detail_re_type: String,

  pub source: String,

  pub fee: i64,

  pub fee_payer: String,

  pub signature: String,

  pub slot: i64,

  pub timestamp: i64,

  pub token_transfers: Vec<TokenTransfer>,

  pub native_transfers: Vec<NativeTransfer>,

  pub account_data: Vec<AccountDatum>,

  pub transaction_error: Option<TransactionError>,
  pub instructions: Vec<Instruction>,

  pub events: Events,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountDatum {
  pub account: String,

  pub native_balance_change: i64,

  pub token_balance_changes: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Events {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
  pub accounts: Option<Vec<String>>,

  pub data: String,

  pub program_id: String,

  pub inner_instructions: Option<Vec<Instruction>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NativeTransfer {
  pub from_user_account: String,

  pub to_user_account: String,

  pub amount: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenTransfer {
  pub from_token_account: String,

  pub to_token_account: String,

  pub from_user_account: String,

  pub to_user_account: String,

  pub token_amount: f64,

  pub mint: String,

  pub token_standard: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionError {
  pub instruction_error: Vec<InstructionErrorElement>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum InstructionErrorElement {
  InstructionErrorClass(InstructionErrorClass),

  Integer(i64),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct InstructionErrorClass {
  pub custom: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayDataItem {
  pub token_mint: String,
  pub sol_mint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RugCheckRes {
  pub token_program: String,

  pub token_type: String,

  pub risks: Vec<RugCheckRisk>,

  pub score: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RugCheckRisk {
  pub name: String,

  pub value: String,

  pub description: String,

  pub score: i64,

  pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QouteResponse {
  pub input_mint: String,

  pub in_amount: String,

  pub output_mint: String,

  pub out_amount: String,

  pub other_amount_threshold: String,

  pub swap_mode: String,

  pub slippage_bps: i64,

  pub platform_fee: Option<serde_json::Value>,

  pub price_impact_pct: String,

  pub route_plan: Vec<RoutePlan>,

  pub score_report: Option<serde_json::Value>,

  pub context_slot: i64,

  pub time_taken: f64,

  pub swap_usd_value: String,

  pub simpler_route_used: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutePlan {
  pub swap_info: SwapInfo,

  pub percent: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapInfo {
  pub amm_key: String,

  pub label: String,

  pub input_mint: String,

  pub output_mint: String,

  pub in_amount: String,

  pub out_amount: String,

  pub fee_amount: String,

  pub fee_mint: String,
}
