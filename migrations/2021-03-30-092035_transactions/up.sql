-- Your SQL goes here

CREATE TABLE transactions (
  id SERIAL PRIMARY KEY,
  input_amount VARCHAR NOT NULL,
  output_amount VARCHAR NOT NULL,
  transaction_address VARCHAR NOT NULL,
  sender_wallet VARCHAR NOT NULL,
  receiver_wallet VARCHAR NOT NULL,
  transaction_signature VARCHAR NOT NULL,
  transaction_type VARCHAR NOT Null,
  transaction_status VARCHAR NOT NULL DEFAULT 'pending',
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
)