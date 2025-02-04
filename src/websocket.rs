use futures::{SinkExt, Stream, StreamExt};
use serde_json;
use thiserror::Error;
use tokio_tungstenite::tungstenite::{protocol::Message, Bytes};

#[derive(Error, Debug)]
pub enum WebsocketError {
  #[error("Connection error: {0}")]
  ConnectionError(String),
  #[error("Subscription error: {0}")]
  SubscriptionError(String),
}

pub struct SolanaWebsocket {
  ws_url: String,
  api_key: String,
}

impl SolanaWebsocket {
  pub fn new(ws_url: &str, api_key: &str) -> Self {
    Self {
      ws_url: ws_url.to_string(),
      api_key: api_key.to_string(),
    }
  }

  pub async fn listen_for_pool_creation(
    &self,
    wallet_address: &str,
  ) -> Result<impl Stream<Item = String>, WebsocketError> {
    let url_string = format!("{}/?api-key={}", self.ws_url, &self.api_key);

    let (ws_stream, _) = tokio_tungstenite::connect_async(url_string) // Pass the String instead of Url
      .await
      .map_err(|e| WebsocketError::ConnectionError(e.to_string()))?;
    let (mut write, read) = ws_stream.split();

    let subscribe_message = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "logsSubscribe",
        "params": [
            {"mentions": [wallet_address]},
            {"commitment": "processed"}
        ]
    });

    // Convert `serde_json::Value` to Utf8Bytes
    let subscribe_message_str = serde_json::to_string(&subscribe_message).map_err(|e| {
      WebsocketError::SubscriptionError(format!("Error serializing message: {}", e))
    })?;

    let subscribe_message_bytes = subscribe_message_str.into_bytes(); // Convert to Utf8Bytes

    write
      .send(Message::Binary(Bytes::from(subscribe_message_bytes)))
      .await
      .map_err(|e| WebsocketError::SubscriptionError(format!("Error listen for socket: {}", e)))?;

    Ok(read.filter_map(|msg| async {
      match msg {
        Ok(Message::Text(text)) => {
          let str = text.to_string();
          Some(str)
        } // `Message::Text` is already a `String`
        _ => None,
      }
    }))
  }
}
