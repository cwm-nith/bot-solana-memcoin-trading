#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub ws_url: String,
    pub telegram_bot_token: String,
}
impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let ws_url = std::env::var("WS_URL").expect("WS_URL must be set");
        let telegram_bot_token =
            std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");

        Config {
            database_url,
            telegram_bot_token,
            ws_url,
        }
    }
}
