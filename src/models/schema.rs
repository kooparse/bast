table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Text,
        created_at -> Timestamp,
    }
}

table! {
    websites (id) {
        id -> Int4,
        user_id -> Int4,
        domain -> Text,
        created_at -> Timestamp,
    }
}

joinable!(websites -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    websites,
);
