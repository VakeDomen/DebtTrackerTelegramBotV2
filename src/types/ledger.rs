use teloxide::types::UserId;
use uuid::Uuid;

#[derive(Debug)]
pub struct Ledger {
    pub id: String,
    pub lender: UserId,
    pub borower: UserId,
    pub sum: i32,
}

#[derive(Debug)]
pub struct SqliteLedger {
    pub id: String,
    pub lender: String,
    pub borower: String,
    pub sum: i32,
}

#[derive(Debug)]
pub struct NewLedger {
    pub lender: String,
    pub borower: String,
    pub sum: i32,
}

impl From<SqliteLedger> for Ledger {
    fn from(ledger: SqliteLedger) -> Self {
        Self { 
            id: ledger.id,
            lender: serde_json::from_str(&ledger.lender).unwrap(), 
            borower: serde_json::from_str(&ledger.borower).unwrap(), 
            sum: ledger.sum 
        }
    }
}

impl From<Ledger> for SqliteLedger {
    fn from(ledger: Ledger) -> Self {
        Self { 
            id: ledger.id,
            lender: serde_json::to_string(&ledger.lender).unwrap(),
            borower: serde_json::to_string(&ledger.borower).unwrap(),
            sum: ledger.sum 
        }   
    }
}

impl From<NewLedger> for SqliteLedger {
    fn from(ledger: NewLedger) -> Self {
        Self { 
            id: Uuid::new_v4().to_string(),
            lender: serde_json::to_string(&ledger.lender).unwrap(),
            borower: serde_json::to_string(&ledger.borower).unwrap(),
            sum: ledger.sum 
        }   
    }
}