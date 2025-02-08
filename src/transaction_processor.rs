use std::time::Duration;

use serde_json::Value;
use solana_sdk::signature::Signer;
use thiserror::Error;

use crate::{
  config::Config,
  model::{StreamMessage, TrxDetailRes},
  websocket::SolanaWebsocket,
};

#[derive(Error, Debug)]
pub enum TransactionError {
  #[error("Invalid transaction format")]
  InvalidTransaction,
  #[error("Signature verification failed")]
  SignatureError,
  #[error("RPC error: {0}")]
  RpcError(String),
}

pub struct TransactionProcessor<'a> {
  config: Config,
  ws: &'a SolanaWebsocket,
}

impl<'a> TransactionProcessor<'a> {
  pub fn new(config: Config, ws: &'a SolanaWebsocket) -> Self {
    Self { config, ws }
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
              self.ws.close();

              println!("======================");
              println!("🔎 New Liquidity Pool found.");
              println!("Puase WS for handle transaction");

              println!("🔃 Fetching transaction details...");
              let signature = signature.as_ref().map_or("", String::as_str);
              let trx_details = self.fetch_trx_details(signature).await;
              match trx_details {
                Ok(trx_details) => {
                  println!("🔎 Transaction details fetched successfully.");
                  println!("🔎 Transaction details: {}", trx_details[0].signature);
                }
                Err(e) => {
                  println!("🔎 Error fetching transaction details: {}", e);
                }
              }
            }
          }
        }
      }
    }
    Ok(())
  }

  async fn fetch_trx_details(&self, signature: &str) -> Result<TrxDetailRes, TransactionError> {
    let mut count_retry = 0;
    let client = reqwest::Client::new();
    let mut response: TrxDetailRes = vec![];
    while count_retry < 3 {
      if count_retry > 0 {
        tokio::time::sleep(Duration::from_secs(10)).await;
        println!("Delay fetching transaction details for 10 secs");
      }
      println!(
        "Retry fetching transaction details... Attempt {}",
        count_retry
      );
      count_retry += 1;
      let json = &serde_json::json!({"transactions": [signature]});
      println!("JSON: {}", json);

      let url = format!(
        "{}/transactions/?api-key={}",
        &self.config.helius_rpc_url, &self.config.helius_api_key
      );

      println!("URL: {}", url);
      let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(json)
        .send()
        .await;

      response = match res {
        Ok(res) => {
          let status = res.status();
          let response_text = res.text().await;
          println!("Status: {}", status);

          match response_text {
            Ok(text) => {
              let data_trx_details = serde_json::from_str::<TrxDetailRes>(&text).unwrap();
              if data_trx_details.len() == 0 {
                continue;
              }
              data_trx_details
            }
            Err(e) => {
              println!("Error fetching transaction details: {}", e);
              continue;
            }
          }
        }
        Err(e) => {
          println!("Error fetching transaction details: {}", e);
          continue;
        }
      };
    }
    Ok(response)
  }

  // async fn get_token_metadata(&self, mint_address: &str) -> Result<Value, TransactionError> {
  //   let client = reqwest::Client::new();
  //   let response = client
  //     .post(&self.rpc_url)
  //     .json(&serde_json::json!({
  //         "jsonrpc": "2.0",
  //         "id": 1,
  //         "method": "getAccountInfo",
  //         "params": [mint_address, {"encoding": "jsonParsed"}]
  //     }))
  //     .send()
  //     .await
  //     .map_err(|e| TransactionError::RpcError(e.to_string()))?;

  //   let result: Value = response
  //     .json()
  //     .await
  //     .map_err(|e| TransactionError::RpcError(e.to_string()))?;

  //   Ok(result)
  // }
}
