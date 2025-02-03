use telegram_bot::{Api, ChatId, Error as TelegramError, SendMessage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotificationError {
  #[error("Telegram API error: {0}")]
  ApiError(#[from] TelegramError),
}

#[derive(Clone)]
pub struct TelegramNotifier {
  api: Api,
  chat_id: ChatId,
}

impl TelegramNotifier {
  pub fn new(token: &str, chat_id: i64) -> Self {
    Self {
      api: Api::new(token),
      chat_id: ChatId::new(chat_id),
    }
  }

  pub async fn send_message(&self, text: &str) -> Result<(), NotificationError> {
    let message = SendMessage::new(self.chat_id, text); // Correct way to create a message
    self
      .api
      .send(message)
      .await
      .map_err(NotificationError::ApiError)?;
    Ok(())
  }
}
