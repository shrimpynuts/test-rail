use crate::{db, models::*, schema::sent_transaction};
use diesel::prelude::*;

pub fn get_sent_transactions() -> Vec<SentTransaction> {
    // use crate::schema::sent_transaction::dsl::*;
    // // TODO: Add to config?
    let connection = &mut db::establish_connection();
    sent_transaction::table
        .load::<SentTransaction>(connection)
        .expect("Error loading accounts")
}
