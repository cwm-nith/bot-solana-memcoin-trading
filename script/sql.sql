CREATE TABLE tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    token TEXT NOT NULL,
    is_sold BOOLEAN NOT NULL,
    buy_at_price REAL NOT NULL,
    sold_at_price REAL NOT NULL,
    balance REAL NOT NULL,
    fee REAL NOT NULL,
    metadata TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

