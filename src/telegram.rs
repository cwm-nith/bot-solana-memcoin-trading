use teloxide::{prelude::*, types::ChatId, RequestError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotificationError {
  #[error("Telegram API error: {0}")]
  ApiError(#[from] RequestError),
}

#[derive(Clone)]
pub struct TelegramNotifier {
  bot: Bot,
  chat_id: ChatId,
}

impl TelegramNotifier {
  pub fn new(token: &str, chat_id: i64) -> Self {
    Self {
      bot: Bot::new(token),
      chat_id: ChatId(chat_id),
    }
  }

  pub async fn send_message(&self, text: &str) -> Result<(), NotificationError> {
    match Requester::send_message(&self.bot, self.chat_id, text)
      .send()
      .await
    {
      Ok(_) => {
        println!("Message sent successfully!");
        Ok(())
      }
      Err(err) => {
        eprintln!("Failed to send message: {:?}", err);
        Err(NotificationError::ApiError(err))
      }
    }
  }
}
