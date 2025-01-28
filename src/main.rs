use config::Config as ConfigApp;
use db::DbClient;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

mod config;
mod db;
mod model;

#[derive(Debug, Clone)]
pub struct AppState {
  pub env: ConfigApp,
  pub db_client: DbClient,
}

async fn init_app() -> AppState {
  dotenv().ok();

  let config = ConfigApp::init();

  let pool = PgPoolOptions::new()
    .max_connections(10)
    .connect(&config.database_url)
    .await
    .expect("Failed to connect to Postgres");

  match sqlx::migrate!().run(&pool).await {
    Ok(_) => {
      println!("Database migration successful");
    }
    Err(e) => {
      eprintln!("Failed to migrate database: {:?}", e);
    }
  }

  let db_client = DbClient::new(pool);
  let app_state = AppState {
    env: config.clone(),
    db_client,
  };
  println!("App initialized successfully");
  app_state
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // WebSocket Connection
  // let ws_url = env::var("DATABASE_URL").expect("WS_URL not set in .env");
  // let db_url = env::var("DB_URL").expect("DB_URL not set in .env");

  //

  // // PostgreSQL connection
  // let db_pool = Arc::new(Mutex::new(Pool::<Postgres>::connect(&db_url).await?));

  // // Telegram bot setup
  // let telegram_bot_token =
  //     env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set in .env");
  // let api = Api::new(telegram_bot_token);

  let app_state = init_app().await;
  println!("App state: {:?}", app_state);
  Ok(())
}

async fn process_transaction(//   log: &RpcTransactionLogs,
//   db_pool: &Arc<Mutex<Pool<Postgres>>>,
//   api: &Api,
) -> Result<(), Box<dyn std::error::Error>> {
  // Extract transaction and do a "Rog Check" (Implement your logic here)
  // let tx = &log.value.signature;
  // println!("Processing transaction: {}", tx);

  // // Save transaction details to PostgreSQL database
  // let mut db = db_pool.lock().await;
  // sqlx::query!(
  //     "INSERT INTO tokens (token, price, balance, fee, metadata, timestamp)
  //      VALUES ($1, $2, $3, $4, $5, $6)",
  //     "example_token",
  //     100.0,
  //     1.0,
  //     0.001,
  //     "metadata_example",
  //     Utc::now()
  // )
  // .execute(&mut *db)
  // .await?;

  // // Send Telegram notification
  // let msg = format!("Transaction: {} processed and saved.", tx);
  // api.send(Message::new("@your_telegram_channel", msg))
  //     .await?;

  Ok(())
}

async fn check_profit_or_loss(//   db_pool: &Arc<Mutex<Pool<Postgres>>>,
//   api: &Api,
) -> Result<(), Box<dyn std::error::Error>> {
  // Fetch all tokens and calculate profit/loss
  // let mut db = db_pool.lock().await;
  // let rows = sqlx::query!("SELECT token, price FROM tokens")
  //     .fetch_all(&mut *db)
  //     .await?;

  // for row in rows {
  //     let profit = calculate_profit(row.price); // Implement your logic
  //     if profit > 1.5 || profit < 0.8 {
  //         // Sell the token
  //         let msg = format!("Token {}: Profit/Loss detected. Selling...", row.token);
  //         api.send(Message::new("@your_telegram_channel", msg))
  //             .await?;
  //     }
  // }

  Ok(())
}

fn calculate_profit(current_price: f64) -> f64 {
  // Replace with your logic to calculate profit/loss
  1.0
}
