use crate::types::{graph::Graph, ledger::Ledger, user::User};
use std::error::Error;

pub fn detect_debt(users: Vec<User>, ledgers: Vec<Ledger>) -> Result<(Vec<Ledger>, Vec<User>), Box<dyn Error>>  {
    let (ledg, usrs) = match detect_mutual_debt(users, ledgers) {
        Ok(state) => state,
        Err(e) => return Err(e),
    };
    let (final_ledg, final_usrs) = match detect_debt_cycles(usrs, ledg) {
        Ok(state) => state,
        Err(e) => return Err(e),
    };
    Ok((final_ledg, final_usrs))
}

pub fn detect_mutual_debt(users: Vec<User>, ledgers: Vec<Ledger>) -> Result<(Vec<Ledger>, Vec<User>), Box<dyn Error>>  {
    let mut graph = Graph::from(users, ledgers);
    match graph.resolve_bidirectional_debt() {
        Ok(_) => (),
        Err(e) => return Err(e)
    }
    graph.reset_nodes();
    
    
    Ok((graph.ledgers, graph.users))
}

pub fn detect_debt_cycles(users: Vec<User>, ledgers: Vec<Ledger>) -> Result<(Vec<Ledger>, Vec<User>), Box<dyn Error>>   {
    let mut graph = Graph::from(users, ledgers);
    match graph.resolve_cyclic_debt() {
        Ok(_) => (),
        Err(e) => return Err(e)
    }
    graph.reset_nodes();
    Ok((graph.ledgers, graph.users))
}