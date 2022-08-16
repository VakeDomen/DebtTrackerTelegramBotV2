use teloxide::types::UserId;
use uuid::Uuid;
use super::schema::ledgers;

#[derive(Debug, Clone)]
pub struct Ledger {
    pub id: String,
    pub borrower: UserId,
    pub owes: UserId,
    pub sum: i32,
}

#[derive(Debug, Insertable, Queryable, Identifiable, AsChangeset)]
#[table_name = "ledgers"]
pub struct SqliteLedger {
    pub id: String,
    pub borrower: String,
    pub owes: String,
    pub sum: i32,
}

#[derive(Debug)]
pub struct NewLedger {
    pub borrower: String,
    pub owes: String,
    pub sum: i32,
}

impl From<SqliteLedger> for Ledger {
    fn from(ledger: SqliteLedger) -> Self {
        Self { 
            id: ledger.id,
            borrower: serde_json::from_str(&ledger.borrower).unwrap(), 
            owes: serde_json::from_str(&ledger.owes).unwrap(), 
            sum: ledger.sum 
        }
    }
}

impl From<Ledger> for SqliteLedger {
    fn from(ledger: Ledger) -> Self {
        Self { 
            id: ledger.id,
            borrower: serde_json::to_string(&ledger.borrower).unwrap(),
            owes: serde_json::to_string(&ledger.owes).unwrap(),
            sum: ledger.sum 
        }   
    }
}

impl From<NewLedger> for SqliteLedger {
    fn from(ledger: NewLedger) -> Self {
        Self { 
            id: Uuid::new_v4().to_string(),
            borrower: ledger.borrower,
            owes: ledger.owes,
            sum: ledger.sum 
        }   
    }
}