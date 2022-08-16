table! {
    ledgers (id) {
        id -> Text,
        borrower -> Text,
        owes -> Text,
        sum -> Integer,
    }
}

table! {
    transactions (id) {
        id -> Text,
        transaction_type -> Text,
        initiator -> Text,
        reciever -> Text,
        sum -> Integer,
        description -> Text,
        created -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        user_id -> Text,
        name -> Text,
    }
}

table! {
    chats (id) {
        id -> Text,
        user_id -> Text,
        chat_id -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    ledgers,
    transactions,
    users,
    chats,
);

joinable!(users -> chats (user_id));