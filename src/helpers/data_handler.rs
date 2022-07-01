
use crate::types::transaction::Transaction;
use diesel::{r2d2::{self, ConnectionManager}, SqliteConnection, Connection};
use r2d2::Pool;
use std::{env};

pub fn fetch_ledger() {

}

pub fn save_transaction(transaction: &Transaction) {
    
}

fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish(
        &env::var("DATABASE_URL").expect("No DATABASE_URL in .env")
    ).expect("Error connecting to database!")
}