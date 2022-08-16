use crate::types::{graph::Graph, ledger::Ledger, user::User};
use std::error::Error;

pub fn detect_mutual_debt(users: Vec<User>, ledgers: Vec<Ledger>) -> Result<(Vec<Ledger>, Vec<User>), Box<dyn Error>>  {
    let mut graph = Graph::from(users, ledgers);
    match graph.resolve_bidirectional_debt() {
        Ok(_) => (),
        Err(e) => return Err(e)
    }
    Ok((graph.ledgers, graph.users))
}

pub fn detect_debt_cycles() {
    
}