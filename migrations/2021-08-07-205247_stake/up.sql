-- Your SQL goes here

CREATE TABLE stakes (
  id SERIAL PRIMARY KEY,
  stake_from VARCHAR NOT NULL,
  stake_signature VARCHAR NOT NULL,
  public_key VARCHAR NOT NULL,
  amount VARCHAR NOT NULL,
  withdraw_day timestamp,
  isActive BOOLEAN NOT NULL DEFAULT 'f',
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
)