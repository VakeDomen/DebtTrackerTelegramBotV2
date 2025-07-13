use std::collections::HashMap;
use std::collections::LinkedList;
use std::vec;
use std::cmp;
use std::error::Error;
use teloxide::types::UserId;

use crate::helpers::data_handler::ledger_operations::update_ledger;

use super::{ledger::Ledger, user::User};

pub struct Graph {
    pub ledgers: Vec<Ledger>,
    pub users: Vec<User>,
    nodes: Vec<Node>
}

#[derive(Debug, Clone)]
struct Node {
    pub id: i32,
    pub user_id: UserId,
    pub connections: Vec<String>,
    pub visited: bool
}

impl Node {
    pub fn new(user_id: UserId, id: i32) -> Node {
        Node { id, user_id, connections: vec![], visited: false }
    }
}

impl Graph {
    pub fn from(users: Vec<User>, ledgers: Vec<Ledger>) -> Self {
        let mut graph = Graph {
            users,
            ledgers,
            nodes: vec![],
        };
        graph.create_nodes();
        graph
    }

    fn create_nodes(&mut self) {
        let mut i = 0;
        for user in self.users.clone().into_iter() {
            let mut node = Node::new(user.user_id, i);
            let ledgers: Vec<String> = self.ledgers.clone().into_iter()
                .filter(|l| l.sum > 0)
                .filter(|l| l.borrower == user.user_id)
                .map(|l| l.id)
                .collect();
            // save the node if it has connections to other nodes
            // connection == non zero sum ledger
            if !ledgers.is_empty() {
                node.connections = ledgers;
                self.nodes.push(node);
                i += 1;
            }
        }
    }

    pub fn resolve_bidirectional_debt(&mut self) -> Result<(), Box<dyn Error>> {
        for node in self.nodes.clone().into_iter() {
            match self.resolve_bidirectional_debt_node(node.clone()) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    pub fn resolve_cyclic_debt(&mut self) -> Result<(), Box<dyn Error>> {
        for node in self.nodes.clone().into_iter() {
            match self.resolve_cyclic_debt_node(node.clone()) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
    fn resolve_cyclic_debt_node(&mut self, node_to_resolve: Node) -> Result<(), Box<dyn Error>> {
        let mut result = LinkedList::new();
        let mut stack = LinkedList::new();
        let mut visited = HashMap::new();
        let to_resolve_uid = node_to_resolve.user_id.to_string();
        let mut found = false;
        
        stack.push_front(node_to_resolve);
        visited.insert(to_resolve_uid.clone(), true);
        
        while !stack.is_empty() {
            found = false;
            let front = match stack.pop_front() {
                Some(n) => n,
                None => break,
            };
            let current = match self.find_node(front.id) {
                Some(n) => n,
                None => continue,
            };
            result.push_front(current.user_id);

            for neighbour_id in current.connections.iter() {
                let neighbour = match self.find_node_by_user_string(neighbour_id) {
                    Some(n) => n,
                    None => continue,
                };
                //if loop closed
                if current.connections.contains(&to_resolve_uid) {
                    result.push_front(neighbour.user_id);
                    found = true;
                    break;
                }
                if !visited.contains_key(neighbour_id) {
                    visited.insert(neighbour_id.to_string(), true);
                    stack.push_front(neighbour);
                }
                if found  {
                    break;
                }
            }
        }
        if found && result.len() > 2 {
            match self.handle_cyclic_debt(result) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
            self.reset_nodes();
        }
        Ok(())
    }

    fn resolve_bidirectional_debt_node(&mut self, node_to_resolve: Node) -> Result<(), Box<dyn Error>> {
        for ledger_id in node_to_resolve.connections.into_iter() {
            // find ledger (connection) to neighbour
            let ledger = match self.find_ledger(&ledger_id) {
                Some(l) => l,
                None => continue,
            };
            // find neighbour node (to be able to reference back to self)
            let neighbour = match self.find_node_by_user(&ledger.owes) {
                Some(n) => n,
                None => continue,
            };
            for neighbour_ledger_id in neighbour.connections.into_iter() {
                // get neighbour ledger (connection) one by one
                let neighbour_ledger = match self.find_ledger(&neighbour_ledger_id) {
                    Some(l) => l,
                    None => continue,
                };
                // if the ledger points back to original node, we have a
                // bidirectional debt
                if neighbour_ledger.owes == ledger.borrower {
                    match handle_bidirectional_ledgers(ledger, neighbour_ledger) {
                        Ok((l1, l2)) => {
                            self.update_ledger(l1);
                            self.update_ledger(l2);
                        },
                        Err(e) => return Err(e),
                    }
                    // continue with next neighbour of the node to resolve
                    break;
                }
            }
            self.ledgers.retain(|l| l.sum != 0);
        }
        Ok(())
    }

    fn update_ledger(&mut self, l: Ledger) {
        for mut ledger in self.ledgers.iter_mut() {
            if ledger.id == l.id {
                ledger.sum = l.sum;
                break;
            }
        }
    }

    fn find_node(&mut self, id: i32) -> Option<Node> {
        for node in self.nodes.clone().into_iter() {
            if node.id == id {
                return Some(node)
            }
        }
        None
    }
    fn find_node_by_user(&mut self, user_id: &UserId) -> Option<Node> {
        for node in self.nodes.clone().into_iter() {
            if node.user_id == *user_id {
                return Some(node)
            }
        }
        None
    }
    fn find_node_by_user_string(&mut self, user_id: &String) -> Option<Node> {
        for node in self.nodes.clone().into_iter() {
            if node.user_id.to_string().eq(user_id) {
                return Some(node)
            }
        }
        None
    }
    fn find_ledger(&mut self, id: &String) -> Option<Ledger> {
        for ledger in self.ledgers.clone().into_iter() {
            if ledger.id.eq(id) {
                return Some(ledger)
            }
        }
        None
    }

    fn find_ledger_by_users(&mut self, borrower: UserId, owes: UserId) -> Option<Ledger> {
        for ledger in self.ledgers.clone().into_iter() {
            if ledger.borrower == borrower && ledger.owes == owes {
                return Some(ledger)
            }
        }
        None
    }

    fn find_user(self, id: &String) -> Option<User> {
        for user in self.users.into_iter() {
            if user.id.eq(id) {
                return Some(user)
            }
        }
        None
    }

    fn handle_cyclic_debt(&mut self, mut result: LinkedList<UserId>) -> Result<(), Box<dyn Error>> {
        let mut current_option = None;
        let mut next_option = None;
        let mut sum = std::i32::MAX;
        while !result.is_empty() {
            current_option = next_option;
            next_option = result.pop_front();

            let current = match current_option {
                Some(uid) => uid,
                None => continue,
            };
            let next = match next_option {
                Some(uid) => uid,
                None => continue,
            };
            let ledger = match self.find_ledger_by_users(current,next) {
                Some(l) => l,
                None => continue,
            };
            sum = cmp::min(sum, ledger.sum);
            
        }
        while !result.is_empty() {
            current_option = next_option;
            next_option = result.pop_front();

            let current = match current_option {
                Some(uid) => uid,
                None => continue,
            };
            let next = match next_option {
                Some(uid) => uid,
                None => continue,
            };
            let ledger = match self.find_ledger_by_users(current,next) {
                Some(l) => l,
                None => continue,
            };
            let updated = match reduce_ledger(ledger, sum) {
                Ok(l) => l,
                Err(e) => return Err(e),
            };
            self.update_ledger(updated);    
        }
        
        Ok(())
    }

    pub fn reset_nodes(&mut self) {
        self.ledgers.retain(|l| l.sum != 0);
        self.nodes = vec![];
        self.create_nodes();
    }
    
}


fn handle_bidirectional_ledgers(l1: Ledger, l2: Ledger) -> Result<(Ledger, Ledger), Box<dyn Error>> {
    // should always be, just double check
    if l1.borrower == l2.owes && l2.borrower == l1.owes {
        let sum = cmp::min(l1.sum, l2.sum);
        let l1_updated = match reduce_ledger(l1, sum) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };
        let l2_updated = match reduce_ledger(l2, sum) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };
        return Ok((l1_updated, l2_updated))
    }   
    Err("ledgers not bidirectioal".into())
}

fn reduce_ledger(mut l: Ledger, sum: i32) -> Result<Ledger, Box<dyn Error>> {
    l.sum -= sum;
    match update_ledger(l) {
        Ok(l) => Ok(l),
        Err(e) => Err(e.into()),
    }
}