# bot-solana-memcoin-trading

## Setup

##### This project needs `openssl` if you don't please install

##### Windows

1.  Install OpenSSL
    Download and install OpenSSL from:
    👉 https://slproweb.com/products/Win32OpenSSL.html
    Install either Win64 OpenSSL (if on 64-bit) or Win32 OpenSSL (for 32-bit systems).

2.  Set Environment Variables
    After installation, set these environment variables:
    - For system-wide settings (run in Command Prompt as Admin):
    ```SHELL
      setx OPENSSL_DIR "C:\Program Files\OpenSSL-Win64"
      setx OPENSSL_INCLUDE_DIR "C:\Program Files\OpenSSL-Win64\include"
      setx OPENSSL_LIB_DIR "C:\Program Files\OpenSSL-Win64\lib"
    ```
    - If installed in a different location, update the path accordingly.

##### For Linux (Ubuntu/Debian)

1. Install OpenSSL Development Libraries

```SHELL
  sudo apt update
  sudo apt install pkg-config libssl-dev
```

2. Check OpenSSL Installation

```SHELL
openssl version -a
```

##### For macOS

1. Install OpenSSL via Homebrew

```SHELL
brew install openssl
```

2. Set Environment Variables

```SHELL
export OPENSSL_DIR=$(brew --prefix openssl)
export OPENSSL_INCLUDE_DIR=$OPENSSL_DIR/include
export OPENSSL_LIB_DIR=$OPENSSL_DIR/lib
```

### Setup Database

```SHELL
// Install sqlx cli
cargo install sqlx-cli --no-default-features --features sqlite

// create database
sqlx database create

// Create a Migration
sqlx migrate add <migration_name>

// Run Migrations
sqlx migrate run

// Rollback Migrations
sqlx migrate revert

// Check Migration Status
sqlx migrate info

```

### Env file

```ENV
DATABASE_URL=sqlite:tokens.db

TELEGRAM_TOKEN=
TELEGRAM_CHAT_ID=

HELIUS_RPC_URL=https://api.helius.xyz/v0
HELIUS_WS_URL=wss://mainnet.helius-rpc.com
HELIUS_API_KEY=

PROGRAM_ID=675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
PRIVATE_KEY=
LIQUIDILITY_POOL_WSOL_PC_MINT=So11111111111111111111111111111111111111112

RUG_CHECKER_URL=https://api.rugcheck.xyz/v1
```

### Run project

```SHELL
// Build project
cargo build

// Run project
cargo run
```

## How It Works

1. Real-Time Listening: The bot listens to Solana logs for liquidity pool creation.
2. Transaction Handling: Each transaction is processed, and details are saved to the database.
3. Price Monitoring: A periodic task checks token prices and calculates profit/loss.
4. Notifications: Telegram bot informs about transaction updates and actions (e.g., sells).
