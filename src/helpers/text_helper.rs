use teloxide::types::UserId;

pub const LOAN_DESCRIPTION: &str = "/loan <amount> <@people>\nThe command loans money to the specified people. The action is constructed from 3 parts (<command> <amount> <poeple>). Amount is a numeric value where the decimal point may be specified with y dot '.' and not comma ','. You may list as many people as you wish as long as they are tagged with a mention (@name). The fee will bi equaly split among the target people.";
pub const PAY_DESCRIPTION: &str = "/pay <amount> <@people>\nThe command will repay the full amount specified to all mentioned people (as long as they are mentioned with @name). If you pay more than you own, the reviever will own you the difference after the transaction completes.";
pub const HISTORY_DESCRIPTION: &str = "/history <number of transactions>\nThe histroy command will display the last completed transactions. You may also specify the amount of transactions displayed, but it defaults to 10 if the argument is not specified.";
pub const BALANCE_DESCRIPTION: &str = "/balance\nThe command will display the current state of debt.";

pub fn generate_transaction_response(sum: i32, reciever: UserId, success: bool) -> String {
    match success {
        true => format!("Successfully loaned {} to {}!", sum, reciever),
        false => "Oops! Something went wrong when processing the transaction! :(".to_string()
    }
}