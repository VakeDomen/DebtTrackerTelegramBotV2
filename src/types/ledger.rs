use super::app_user::AppUser;

pub struct Ledger {
    pub lender: AppUser,
    pub borower: AppUser,
    pub sum: i64,
}