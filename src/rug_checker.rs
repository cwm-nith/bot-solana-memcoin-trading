use crate::{config::Config, model::RugCheckRes};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RugCheckerError {
  #[error("Connection error: {0}")]
  RugCheckError(String),
}

pub struct RugChecker<'a> {
  pub config: &'a Config,
}

impl<'a> RugChecker<'a> {
  pub fn new(config: &'a Config) -> Self {
    Self { config }
  }

  pub async fn isvalid_rug_check(&self, token_mint: &str) -> Result<bool, RugCheckerError> {
    let client = reqwest::Client::new();
    let url = format!(
      "{}/tokens/{}/report/summary",
      self.config.rug_checker_url, token_mint
    );
    let res = client
      .get(url)
      .header("Content-Type", "application/json")
      .send()
      .await;

    match res {
      Ok(data) => {
        let status = data.status();
        let response_txt = data.text().await;
        println!("Status: {}", status);

        match response_txt {
          Ok(txt) => {
            println!("Response: {}", txt);
            let res_data = serde_json::from_str::<RugCheckRes>(&txt).unwrap();
            let mut is_valid = true;
            res_data.risks.iter().for_each(|risk| {
              println!("Rug: {:?}", risk);
              if risk.name.to_lowercase() == "single holder ownership" {
                let value = risk.value.replace("%", "");
                let numberic_value = value.parse::<f64>().unwrap();
                if numberic_value > self.config.rug_check_config.signal_holder_ownership {
                  is_valid = false;
                }
              }
            });
            if !is_valid {
              return Ok(false);
            }

            let res_data = res_data.clone();
            let valid = !res_data.risks.iter().any(|r| {
              self
                .config
                .rug_check_config
                .not_allowed_risk
                .contains(&r.name)
            });
            return Ok(valid);
          }
          Err(e) => {
            return Err(RugCheckerError::RugCheckError(e.to_string()));
          }
        }
      }
      Err(e) => {
        return Err(RugCheckerError::RugCheckError(e.to_string()));
      }
    };
  }
}
