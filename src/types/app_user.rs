use teloxide::types::UserId;

pub enum AppUser {
    Telegram(UserId)
}