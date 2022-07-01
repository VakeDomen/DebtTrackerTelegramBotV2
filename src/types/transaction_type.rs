use std::fmt;

#[derive(Debug)]
pub enum TransactionType {
    Loan,
    Payment,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<String> for TransactionType {
    fn from(string_type: String) -> Self {
        match string_type.as_str() {
            "PAYMENT" => TransactionType::Payment,
            "LOAN" => TransactionType::Loan,
            _ => TransactionType::Loan
        }
    }
}
