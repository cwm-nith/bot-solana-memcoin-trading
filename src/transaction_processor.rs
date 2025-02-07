use serde_json::Value;
use solana_sdk::signature::{Signature, Signer};
use thiserror::Error;

use crate::model::StreamMessage;

#[derive(Error, Debug)]
pub enum TransactionError {
  #[error("Invalid transaction format")]
  InvalidTransaction,
  #[error("Signature verification failed")]
  SignatureError,
  #[error("RPC error: {0}")]
  RpcError(String),
}

pub struct TransactionProcessor {
  rpc_url: String,
}

impl TransactionProcessor {
  pub fn new(rpc_url: &str) -> Self {
    Self {
      rpc_url: rpc_url.to_string(),
    }
  }

  pub async fn process_transaction(
    &self,
    tx_data: &str,
    _signer: &dyn Signer,
  ) -> Result<(), TransactionError> {
    // let tx: Value =
    //   serde_json::from_str(tx_data).map_err(|_| TransactionError::InvalidTransaction)?;

    // // Extract required fields
    // let signature_str = tx["signature"]
    //   .as_str()
    //   .ok_or(TransactionError::InvalidTransaction)?;

    // let message = tx["message"]
    //   .as_str()
    //   .ok_or(TransactionError::InvalidTransaction)?;

    // // Convert signature string to `Signature`
    // let signature = signature_str
    //   .parse::<Signature>()
    //   .map_err(|_| TransactionError::InvalidTransaction)?;

    // // Verify the signature using Solana's built-in method
    // let pubkey = signer.pubkey();
    // if !signature.verify(pubkey.as_ref(), message.as_bytes()) {
    //   return Err(TransactionError::SignatureError);
    // }

    let data = serde_json::from_str::<StreamMessage>(tx_data).unwrap();

    if let Some(params) = &data.params {
      if let Some(result) = &params.result {
        if let Some(value) = &result.value {
          let log = &value.logs;
          let signature = &value.signature;

          if signature.is_none() {
            return Err(TransactionError::SignatureError);
          }

          if let Some(logs) = log {
            let contains_create = logs
              .iter()
              .find(|x| x.starts_with("Program log: initialize2: InitializeInstruction2"));
            if contains_create.is_some() {
              println!(
                "Create pool transaction detected: {}",
                serde_json::to_string(&data).unwrap()
              );
            }
          }
        }
      }
    }
    Ok(())
  }

  async fn get_token_metadata(&self, mint_address: &str) -> Result<Value, TransactionError> {
    let client = reqwest::Client::new();
    let response = client
      .post(&self.rpc_url)
      .json(&serde_json::json!({
          "jsonrpc": "2.0",
          "id": 1,
          "method": "getAccountInfo",
          "params": [mint_address, {"encoding": "jsonParsed"}]
      }))
      .send()
      .await
      .map_err(|e| TransactionError::RpcError(e.to_string()))?;

    let result: Value = response
      .json()
      .await
      .map_err(|e| TransactionError::RpcError(e.to_string()))?;

    Ok(result)
  }
}
