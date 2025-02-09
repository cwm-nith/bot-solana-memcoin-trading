#[derive(Debug, Clone, PartialEq)]
pub struct Config {
  pub database_url: String,
  pub telegram_token: String,
  pub telegram_chat_id: i64,

  pub helius_rpc_url: String,
  pub helius_ws_url: String,
  pub helius_api_key: String,

  pub program_id: String,
  pub private_key: String,
  pub liquidility_pool_wsol_pc_mint: String,
  pub rug_checker_url: String,
  pub rug_check_config: RugCheckConfig,
  pub swap_config: SwapConfig,
  pub jupiter_url: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RugCheckConfig {
  pub signal_holder_ownership: f64,
  pub not_allowed_risk: Vec<String>,
  pub is_skip_pump_token: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwapConfig {
  pub amount: String, // lamports
  pub slippage_bps: String,
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
    let liquidility_pool_wsol_pc_mint = std::env::var("LIQUIDILITY_POOL_WSOL_PC_MINT")
      .expect("LIQUIDILITY_POOL_WSOL_PC_MINT must be set");
    let rug_checker_url = std::env::var("RUG_CHECKER_URL").expect("RUG_CHECKER_URL must be set");
    let jupiter_url = std::env::var("JUPITER_URL").expect("JUPITER_URL must be set");

    Config {
      database_url,
      telegram_token,
      helius_ws_url,
      helius_api_key,
      helius_rpc_url,
      private_key,
      program_id,
      telegram_chat_id: telegram_chat_id.parse::<i64>().unwrap(),
      liquidility_pool_wsol_pc_mint,
      rug_checker_url,
      rug_check_config: RugCheckConfig {
        signal_holder_ownership: 0 as f64,
        not_allowed_risk: vec![
          "Freeze Authority still enabled".to_string(),
          "Large Amount of LP Unlocked".to_string(),
          "Copycat token".to_string(),
        ],
        is_skip_pump_token: true,
      },
      swap_config: SwapConfig {
        amount: "10000000000".to_string(), // 0.01 SOL = 10000000000 lamports
        slippage_bps: "200".to_string(),
      },
      jupiter_url,
    }
  }
}
