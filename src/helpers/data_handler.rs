
pub mod ledger_operations {
    use diesel::{QueryDsl, result::Error, insert_into};
    use diesel::prelude::*;
    use teloxide::types::UserId;
    use crate::types::{schema::ledgers::dsl::*, ledger::Ledger};
    use crate::helpers::data_handler::sqlite_operations::establish_connection;
    use crate::types::ledger::{SqliteLedger, NewLedger};

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
        let _ = insert_into(ledgers)
            .values(&sqlite_ledger)
            .execute(&conn)?;
        Ok(Ledger::from(sqlite_ledger))
    }
}
    
pub mod transaction_operations {
    use diesel::{result::Error, insert_into};
    use diesel::prelude::*;
    use crate::types::transaction::{NewTransaction, Transaction, SqliteTransaction};
    use crate::helpers::data_handler::sqlite_operations::establish_connection;
    use crate::types::schema::transactions::dsl::*;
    
    pub fn insert_transaction(new_transaction: NewTransaction) -> Result<Transaction, Error>  {
        let sqlite_transaction = SqliteTransaction::from(new_transaction);
        let conn = establish_connection();
        let _ = insert_into(transactions)
            .values(&sqlite_transaction)
            .execute(&conn)?;
        Ok(Transaction::from(sqlite_transaction))
    }
}

pub mod user_operations {
    use diesel::{result::Error, insert_into};
    use diesel::prelude::*;
    use teloxide::types::UserId;
    use crate::helpers::data_handler::sqlite_operations::establish_connection;
    use crate::types::schema::users::dsl::*;
    use crate::types::user::{User, SqliteUser, NewUser};
    
    pub fn get_user_by_user_id(query_id: &UserId) -> Result<Vec<User>, Error> {
        let conn = establish_connection();
        let resp = users
            .filter(user_id.eq(query_id.to_string()))
            .load::<SqliteUser>(&conn)?;
        Ok(resp.into_iter().map(|u| User::from(u)).collect())
    }
    
    pub fn get_user_by_username(query_username: String) -> Result<Vec<User>, Error> {
        let conn = establish_connection();
        let resp = users
            .filter(name.eq(query_username))
            .load::<SqliteUser>(&conn)?;
        Ok(resp.into_iter().map(|u| User::from(u)).collect())
    }
    
    pub fn insert_user(new_user: NewUser) -> Result<User, Error>  {
        let sqlite_user = SqliteUser::from(new_user);
        let conn = establish_connection();
        let _ = insert_into(users)
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
}

pub mod chat_operations {
    use diesel::{result::Error, insert_into};
    use diesel::prelude::*;
    use teloxide::types::{UserId, ChatId};
    use crate::types::chat::{Chat, NewChat, SqliteChat};
    use crate::helpers::data_handler::sqlite_operations::establish_connection;
    use crate::types::schema::chats::dsl::*;
    
    pub fn insert_user_into_room(uid: &UserId, cid: &ChatId) -> Result<Chat, Error> {
        let sqlite_chat = SqliteChat::from(NewChat {
            user_id: *uid,
            chat_id: *cid
        });
        let conn = establish_connection();
        let _ = insert_into(chats)
            .values(&sqlite_chat)
            .execute(&conn)?;
        Ok(Chat::from(sqlite_chat))
    }

    pub fn is_user_in_chat(uid: UserId, cid: ChatId) -> Result<bool, Error> {
        let conn = establish_connection();
        let resp = chats
            .filter(user_id.eq(uid.to_string()))
            .filter(chat_id.eq(cid.to_string()))
            .load::<SqliteChat>(&conn)?;
        Ok(resp.len() > 0)
    }
}

pub mod sqlite_operations {
    use diesel::{SqliteConnection, Connection};
    use std::{env};
    pub(crate) fn establish_connection() -> SqliteConnection {
        SqliteConnection::establish(
            &env::var("DATABASE_URL").expect("No DATABASE_URL in .env")
        ).expect("Error connecting to database!")
    }
}