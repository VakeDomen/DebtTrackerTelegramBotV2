use teloxide::types::UserId;
use teloxide::types::ChatId;
use uuid::Uuid;
use super::schema::chats;
use super::user::SqliteUser;

#[derive(Debug)]
pub struct Chat {
    pub id: String,
    pub user_id: UserId,
    pub chat_id: ChatId,
}

#[derive(Debug, Insertable, Queryable, Identifiable, AsChangeset)]
#[table_name = "chats"]
pub struct SqliteChat {
    pub id: String,
    pub user_id: String,
    pub chat_id: String
}

pub struct NewChat {
    pub user_id: UserId,
    pub chat_id: ChatId
}

impl From<SqliteChat> for Chat {
    fn from(chat: SqliteChat) -> Self {
        Self { 
            id: chat.id, 
            user_id: serde_json::from_str(&chat.user_id).unwrap(),
            chat_id: serde_json::from_str(&chat.chat_id).unwrap(),
        }
    }
}

impl From<Chat> for SqliteChat {
    fn from(chat: Chat) -> Self {
        Self { 
            id: chat.id, 
            user_id: chat.user_id.to_string(),
            chat_id: chat.chat_id.to_string(),
        }
    }
}

impl From<NewChat> for SqliteChat {
    fn from(chat: NewChat) -> Self {
        Self { 
            id: Uuid::new_v4().to_string(), 
            user_id: chat.user_id.to_string(),
            chat_id: chat.chat_id.to_string(),
        }
    }
}
