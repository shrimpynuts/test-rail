// @generated automatically by Diesel CLI.

diesel::table! {
    managed_account (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        address -> Text,
    }
}

diesel::table! {
    sent_transaction (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        account -> Text,
        hash -> Text,
        gas -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    managed_account,
    sent_transaction,
);
