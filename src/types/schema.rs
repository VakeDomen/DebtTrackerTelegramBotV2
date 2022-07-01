table! {
    ledgers (id) {
        id -> Text,
        lender -> Text,
        borower -> Text,
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
        chat_id -> Text,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    ledgers,
    transactions,
    users,
);
