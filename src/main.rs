mod config;
mod database;
mod price_monitor;
mod telegram;
mod transaction_processor;
mod websocket;

use crate::{
  database::Database, price_monitor::PriceMonitor, telegram::TelegramNotifier,
  websocket::SolanaWebsocket,
};
use futures::StreamExt;
use solana_sdk::signer::keypair::Keypair;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv::dotenv().ok();

  let config = config::Config::init();
  let notifier = TelegramNotifier::new(&config.telegram_token, config.telegram_chat_id);

  // Initialize components
  let db = Database::new(&config.database_url).await?;

  let price_monitor = PriceMonitor::new(db.clone(), notifier.clone(), &config.helius_rpc_url);

  // Start price monitoring
  tokio::spawn(async move {
    price_monitor.start_monitoring().await;
  });

  // Initialize WebSocket listener
  let websocket = SolanaWebsocket::new(&config.helius_ws_url, &config.helius_api_key);
  let stream = websocket
    .listen_for_pool_creation(&config.program_id)
    .await?;

  let mut stream = Box::pin(stream);

  while let Some(message) = stream.next().await {
    println!("New transaction: {}", message);

    // Process transaction
    let signer = Keypair::from_base58_string(&config.private_key);
    let processor = transaction_processor::TransactionProcessor::new(&config.helius_rpc_url);

    // let notifier = notifier.clone();
    if let Err(e) = processor.process_transaction(&message, &signer).await {
      eprintln!("Error processing transaction: {}", e);
      notifier
        .send_message(&format!("Error processing transaction: {}", e))
        .await?;
    }
  }

  Ok(())
}
