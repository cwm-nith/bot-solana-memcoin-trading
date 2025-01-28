# bot-solana-memcoin-trading

## How It Works

1. Real-Time Listening: The bot listens to Solana logs for liquidity pool creation.
2. Transaction Handling: Each transaction is processed, and details are saved to the database.
3. Price Monitoring: A periodic task checks token prices and calculates profit/loss.
4. Notifications: Telegram bot informs about transaction updates and actions (e.g., sells).
