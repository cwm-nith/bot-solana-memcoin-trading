Bot Snaper Solana Memecoin

These are the list points if steps to do.

- Listen on WebSocket RPC Logs for finding liquidity swap pool created
- Get Trx and do the Rog Check
- Mapping document for swapping the token and sign the wallet
- Once the swap success, get trx detail like Balance, Price, Fee and Metadata to save in Sql DB
- Send Trx detail to telegram bot
- Loop for every 5s get all the tokens that saved in db to check current price
- If the profit gant 150% sell the token Send Trx detail to telegram bot
- If the profit loss 20% sell the token Send Trx detail to telegram bot

Do it in Rust
