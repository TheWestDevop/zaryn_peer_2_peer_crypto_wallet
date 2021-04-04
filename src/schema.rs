table! {
    transactions (id) {
        id -> Int4,
        input_amount -> Varchar,
        output_amount -> Varchar,
        transaction_address -> Varchar,
        sender_wallet -> Varchar,
        receiver_wallet -> Varchar,
        transaction_signature -> Varchar,
        transaction_type -> Varchar,
        transaction_status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    wallets (id) {
        id -> Int4,
        wallet_address -> Varchar,
        private_key -> Varchar,
        public_key -> Varchar,
        amount -> Varchar,
        wallet_value -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    transactions,
    wallets,
);
