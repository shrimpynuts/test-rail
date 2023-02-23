use crate::{db, models::*, schema::managed_account};
use diesel::prelude::*;

pub fn get_managed_accounts() -> Vec<ManagedAccount> {
    // use crate::schema::managed_account::dsl::*;
    // // TODO: Add to config?
    let connection = &mut db::establish_connection();
    managed_account::table
        .load::<ManagedAccount>(connection)
        .expect("Error loading accounts")
}
