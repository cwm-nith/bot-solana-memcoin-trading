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
use std::env; // Import StreamExt to use `next`

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv::dotenv().ok();

  // Initialize components
  let db = Database::new("sqlite:tokens.db").await?;
  let notifier = TelegramNotifier::new(
    &env::var("TELEGRAM_TOKEN")?,
    env::var("TELEGRAM_CHAT_ID")?.parse()?,
  );

  let price_monitor = PriceMonitor::new(db.clone(), notifier.clone(), &env::var("SOLANA_RPC_URL")?);

  // Start price monitoring
  tokio::spawn(async move {
    price_monitor.start_monitoring().await;
  });

  // Initialize WebSocket listener
  let websocket = SolanaWebsocket::new(&env::var("SOLANA_WS_URL")?);
  let stream = websocket
    .listen_for_pool_creation(&env::var("WALLET_ADDRESS")?)
    .await?;

  // Pin the stream to avoid the `cannot be unpinned` error
  let mut stream = Box::pin(stream);
  println!("New transaction");

  let test = stream.next().await;
  println!("New transaction: {}", test.unwrap());
  // Process incoming transactions
  while let Some(message) = stream.next().await {
    println!("New transaction: {}", message);

    // Process transaction
    let signer = Keypair::from_base58_string(&env::var("PRIVATE_KEY")?);
    let processor = transaction_processor::TransactionProcessor::new(&env::var("SOLANA_RPC_URL")?);

    if let Err(e) = processor.process_transaction(&message, &signer).await {
      notifier
        .send_message(&format!("Error processing transaction: {}", e))
        .await
        .ok();
    }
  }

  Ok(())
}
