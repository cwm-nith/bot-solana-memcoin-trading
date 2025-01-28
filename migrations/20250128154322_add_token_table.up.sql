-- Add up migration script here

CREATE TABLE tokens (
    id serial primary key,
    token TEXT NOT NULL,
    is_sold BOOLEAN NOT NULL,
    buy_at_price REAL NOT NULL,
    sold_at_price REAL NOT NULL,
    balance REAL NOT NULL,
    fee REAL NOT NULL,
    metadata TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX users_token_index ON tokens(token);