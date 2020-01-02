table! {
    ghosts (id) {
        id -> Int4,
        user_id -> Int4,
        website_id -> Int4,
        is_new_session -> Bool,
        pathname -> Text,
        hostname -> Text,
        referrer -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    pages (id) {
        id -> Int4,
        website_id -> Int4,
        pathname -> Text,
        visitors -> Int4,
        sessions -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    websites (id) {
        id -> Int4,
        user_id -> Int4,
        visitors -> Int4,
        sessions -> Int4,
        domain -> Text,
        created_at -> Timestamp,
    }
}

joinable!(ghosts -> users (user_id));
joinable!(ghosts -> websites (website_id));
joinable!(pages -> websites (website_id));
joinable!(websites -> users (user_id));

allow_tables_to_appear_in_same_query!(ghosts, pages, users, websites,);
