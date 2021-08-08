
table! {
    schema_migrations (version) {
        version -> Int8,
        inserted_at -> Nullable<Timestamp>,
    }
}

table! {
    stakes (id) {
        id -> Int4,
        stake_from -> Varchar,
        stake_signature -> Varchar,
        public_key -> Varchar,
        amount -> Varchar,
        withdraw_day -> Nullable<Timestamp>,
        isactive -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    transactions (id) {
        id -> Int4,
        amount -> Varchar,
        transaction_address -> Varchar,
        sender_wallet -> Varchar,
        receiver_wallet -> Varchar,
        transaction_signature -> Varchar,
        transaction_type -> Varchar,
        transaction_fee -> Varchar,
        transaction_status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    wallets (id) {
        id -> Int4,
        wallet_address -> Varchar,
        wallet_signature -> Varchar,
        public_key -> Varchar,
        amount -> Varchar,
        wallet_value -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}


allow_tables_to_appear_in_same_query!(
    
    schema_migrations,
    stakes,
    transactions,
    wallets,
);
