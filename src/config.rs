#[derive(Debug, Clone)]
pub struct Config {
  pub database_url: String,
  pub telegram_token: String,
  pub telegram_chat_id: i64,

  pub helius_rpc_url: String,
  pub helius_ws_url: String,
  pub helius_api_key: String,

  pub program_id: String,
  pub private_key: String,
}
impl Config {
  pub fn init() -> Config {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let helius_ws_url = std::env::var("HELIUS_WS_URL").expect("HELIUS_WS_URL must be set");
    let telegram_token = std::env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN must be set");
    let telegram_chat_id = std::env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID must be set");
    let helius_rpc_url = std::env::var("HELIUS_RPC_URL").expect("HELIUS_RPC_URL must be set");
    let helius_api_key = std::env::var("HELIUS_API_KEY").expect("HELIUS_API_KEY must be set");
    let program_id = std::env::var("PROGRAM_ID").expect("PROGRAM_ID must be set");
    let private_key = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");

    Config {
      database_url,
      telegram_token,
      helius_ws_url,
      helius_api_key,
      helius_rpc_url,
      private_key,
      program_id,
      telegram_chat_id: telegram_chat_id.parse::<i64>().unwrap(),
    }
  }
}
