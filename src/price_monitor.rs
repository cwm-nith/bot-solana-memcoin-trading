use crate::database::Database;
use crate::telegram::TelegramNotifier;
use std::time::Duration;
use tokio::time;

pub struct PriceMonitor {
  db: Database,
  notifier: TelegramNotifier,
  rpc_url: String,
}

impl PriceMonitor {
  pub fn new(db: Database, notifier: TelegramNotifier, rpc_url: &str) -> Self {
    Self {
      db,
      notifier,
      rpc_url: rpc_url.to_string(),
    }
  }

  pub async fn start_monitoring(&self) {
    let mut interval = time::interval(Duration::from_secs(5));

    loop {
      interval.tick().await;

      if let Ok(tokens) = self.db.get_all_tokens().await {
        for token in tokens {
          if let Ok(current_price) = self.get_current_price(&token.mint_address).await {
            let entry_price = token.entry_price;
            let profit = (current_price - entry_price) / entry_price * 100.0;

            if profit >= 150.0 {
              self
                .notifier
                .send_message(&format!(
                  "🚀 Profit 150%+ detected! Selling {} at {}",
                  token.mint_address, current_price
                ))
                .await
                .ok();
              // Implement sell logic
            } else if profit <= -20.0 {
              self
                .notifier
                .send_message(&format!(
                  "⚠️ 20% loss detected! Selling {} at {}",
                  token.mint_address, current_price
                ))
                .await
                .ok();
              // Implement sell logic
            }
          }
        }
      }
    }
  }

  async fn get_current_price(&self, mint_address: &str) -> Result<f64, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
      .post(&self.rpc_url)
      .json(&serde_json::json!({
          "jsonrpc": "2.0",
          "id": 1,
          "method": "getTokenAccountBalance",
          "params": [mint_address]
      }))
      .send()
      .await?;

    let result: serde_json::Value = response.json().await?;
    Ok(
      result["result"]["value"]["uiAmount"]
        .as_f64()
        .unwrap_or(0.0),
    )
  }
}
