-- Add migration script here
CREATE TABLE tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mint_address TEXT NOT NULL,
    balance REAL NOT NULL,
    entry_price REAL NOT NULL,
    fees REAL NOT NULL,
    metadata TEXT NOT NULL,
    timestamp INTEGER NOT NULL
);
-- Create index for faster price checks
CREATE INDEX IF NOT EXISTS idx_mint_address ON tokens(mint_address);