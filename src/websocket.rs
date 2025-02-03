use futures::{SinkExt, Stream, StreamExt};
use serde_json;
use thiserror::Error;
use tokio_tungstenite::tungstenite::{protocol::Message, Bytes};
use url::Url; // Make sure to include this import for StreamExt

#[derive(Error, Debug)]
pub enum WebsocketError {
  #[error("Connection error: {0}")]
  ConnectionError(String),
  #[error("Subscription error: {0}")]
  SubscriptionError(String),
  #[error("WebSocket error: {0}")]
  WebSocketError(String),
}

pub struct SolanaWebsocket {
  ws_url: String,
}

impl SolanaWebsocket {
  pub fn new(ws_url: &str) -> Self {
    Self {
      ws_url: ws_url.to_string(),
    }
  }

  pub async fn listen_for_pool_creation(
    &self,
    wallet_address: &str,
  ) -> Result<impl Stream<Item = String>, WebsocketError> {
    let url = Url::parse(&self.ws_url)
      .map_err(|e| WebsocketError::ConnectionError(format!("Invalid URL: {}", e)))?;

    // Convert Url to String to fulfill the IntoClientRequest trait requirement
    let url_string = url.to_string();

    let (ws_stream, _) = tokio_tungstenite::connect_async(url_string) // Pass the String instead of Url
      .await
      .map_err(|e| WebsocketError::ConnectionError(e.to_string()))?;
    println!("Connected to Solana WebSocket");
    let (mut write, read) = ws_stream.split();

    let subscribe_message = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "logsSubscribe",
        "params": [
            {"mentions": [wallet_address]},
            {"commitment": "confirmed"}
        ]
    });

    // Convert `serde_json::Value` to Utf8Bytes
    let subscribe_message_str = serde_json::to_string(&subscribe_message).map_err(|e| {
      WebsocketError::SubscriptionError(format!("Error serializing message: {}", e))
    })?;

    println!(
      "Subscribing to pool creation events: {}",
      subscribe_message_str
    );

    let subscribe_message_bytes = subscribe_message_str.into_bytes(); // Convert to Utf8Bytes

    write
      .send(Message::Binary(Bytes::from(subscribe_message_bytes)))
      .await
      .map_err(|e| WebsocketError::SubscriptionError(e.to_string()))?;

    Ok(read.filter_map(|msg| async {
      match msg {
        Ok(Message::Text(text)) => {
          let str = text.to_string();
          println!("Received message: {}", str);
          Some(str)
        } // `Message::Text` is already a `String`
        _ => None,
      }
    }))
  }
}
