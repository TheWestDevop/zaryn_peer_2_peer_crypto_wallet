-- Your SQL goes here

CREATE TABLE wallets (
  id SERIAL PRIMARY KEY,
  wallet_address VARCHAR NOT NULL,
  wallet_signature VARCHAR NOT NULL,
  public_key VARCHAR NOT NULL,
  amount VARCHAR NOT NULL,
  wallet_value VARCHAR NOT NULL,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
)