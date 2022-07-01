
use crate::types::{transaction::{Transaction, NewTransaction, SqliteTransaction}, ledger::{Ledger, SqliteLedger, NewLedger}};
use diesel::{SqliteConnection, Connection, QueryDsl, result::Error, insert_into};
use teloxide::types::UserId;
use std::{env};
use diesel::prelude::*;
use crate::types::schema::ledgers::dsl::*;
use crate::types::schema::transactions::dsl::*;

pub fn get_ledger(bor: &UserId, owe: &UserId) -> Result<Vec<Ledger>, Error> {
    let connection = establish_connection();
    
    let bow_string = serde_json::to_string(&bor).unwrap();
    let owe_string = serde_json::to_string(&owe).unwrap();
    
    let ledger = ledgers
        .filter(borrower.eq(bow_string))
        .filter(owes.eq(owe_string))
        .load::<SqliteLedger>(&connection)?;
    Ok(ledger.into_iter().map(|l| Ledger::from(l)).collect())
}

pub fn update_ledger(ledger: Ledger) -> Result<Ledger, Error> {
    let sqlite_ledger = SqliteLedger::from(ledger);
    let conn = establish_connection();
    let updated_ledger: SqliteLedger = sqlite_ledger.save_changes::<SqliteLedger>(&conn)?;
    Ok(Ledger::from(updated_ledger))
}

pub fn insert_ledger(new_ledger: NewLedger) -> Result<Ledger, Error> {
    let sqlite_ledger = SqliteLedger::from(new_ledger);
    let conn = establish_connection();
    let resp = insert_into(ledgers)
        .values(&sqlite_ledger)
        .execute(&conn)?;
    Ok(Ledger::from(sqlite_ledger))
}

pub fn insert_transaction(new_transaction: NewTransaction) -> Result<Transaction, Error>  {
    let sqlite_transaction = SqliteTransaction::from(new_transaction);
    let conn = establish_connection();
    let resp = insert_into(transactions)
        .values(&sqlite_transaction)
        .execute(&conn)?;
    Ok(Transaction::from(sqlite_transaction))
}

fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish(
        &env::var("DATABASE_URL").expect("No DATABASE_URL in .env")
    ).expect("Error connecting to database!")
}