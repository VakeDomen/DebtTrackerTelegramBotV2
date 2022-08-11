
use crate::types::{transaction::{Transaction, NewTransaction, SqliteTransaction}, ledger::{Ledger, SqliteLedger, NewLedger}, user::{SqliteUser, NewUser, User}};
use diesel::{SqliteConnection, Connection, QueryDsl, result::Error, insert_into};
use teloxide::types::UserId;
use std::{env};
use diesel::prelude::*;
use crate::types::schema::ledgers::dsl::*;
use crate::types::schema::transactions::dsl::*;
use crate::types::schema::users::dsl::*;

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

pub fn get_user_by_user_id(query_id: &UserId) -> Result<Vec<User>, Error> {
    let conn = establish_connection();
    let resp = users
        .filter(user_id.eq(query_id.to_string()))
        .load::<SqliteUser>(&conn)?;
    Ok(resp.into_iter().map(|u| User::from(u)).collect())
}

pub fn insert_user(new_user: NewUser) -> Result<User, Error>  {
    let sqlite_user = SqliteUser::from(new_user);
    let conn = establish_connection();
    let resp = insert_into(users)
        .values(&sqlite_user)
        .execute(&conn)?;
    Ok(User::from(sqlite_user))
}

pub fn update_user(user: User) -> Result<User, Error> {
    let sqlite_user = SqliteUser::from(user);
    let conn = establish_connection();
    let updated_user: SqliteUser = sqlite_user.save_changes::<SqliteUser>(&conn)?;
    Ok(User::from(updated_user))
}

fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish(
        &env::var("DATABASE_URL").expect("No DATABASE_URL in .env")
    ).expect("Error connecting to database!")
}