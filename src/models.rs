use diesel::prelude::*;

// Rust structs that map to the database tables
// Diesel ORM will deserialize the database rows into these structs
// Rust to SQL type bindings: https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html

#[derive(Queryable, Debug)]
pub struct ManagedAccount {
    pub id: uuid::Uuid,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub address: String,
}

#[derive(Queryable, Debug)]
pub struct SentTransaction {
    pub id: i64,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub account: String,
    pub hash: String,
    pub gas: i64,
}
