use teloxide::types::UserId;
use uuid::Uuid;
use super::schema::users;

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub user_id: UserId,
    pub username: String
}

#[derive(Debug, Insertable, Queryable, Identifiable, AsChangeset)]
#[table_name = "users"]
pub struct SqliteUser {
    pub id: String,
    pub user_id: String,
    pub name: String
}

pub struct NewUser {
    pub user_id: UserId,
    pub username: String
}

impl From<SqliteUser> for User {
    fn from(user: SqliteUser) -> Self {
        Self { 
            id: user.id, 
            user_id: serde_json::from_str(&user.user_id).unwrap(),
            username: user.name
        }
    }
}

impl From<&teloxide::types::User> for NewUser {
    fn from(user: &teloxide::types::User) -> Self {
        Self {
            user_id: user.id,
            username: user.username.as_ref().expect("Unable to determine username").to_string()
        }
    }
}

impl From<User> for SqliteUser {
    fn from(user: User) -> Self {
        Self { 
            id: user.id, 
            user_id: user.user_id.to_string(),
            name: user.username
        }
    }
}

impl From<NewUser> for SqliteUser {
    fn from(user: NewUser) -> Self {
        Self { 
            id: Uuid::new_v4().to_string(), 
            user_id: user.user_id.to_string(),
            name: user.username
        }
    }
}
