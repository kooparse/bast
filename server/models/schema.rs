table! {
    pageviews (id) {
        id -> Uuid,
        website_id -> Int4,
        u_id -> Text,
        pathname -> Text,
        href -> Text,
        hostname -> Text,
        referrer -> Text,
        user_agent -> Text,
        location -> Nullable<Text>,
        is_new_session -> Bool,
        is_new_user -> Bool,
        duration -> Float4,
        is_bounce -> Bool,
        is_done -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    stats (id) {
        id -> Int4,
        website_id -> Int4,
        pageviews -> Int4,
        users -> Int4,
        sessions -> Int4,
        avg_time -> Float4,
        bounce_rate -> Float4,
        known_time_counter -> Int4,
        bounce_counter -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

table! {
    websites (id) {
        id -> Int4,
        user_id -> Int4,
        domain -> Text,
        pageviews -> Int4,
        users -> Int4,
        sessions -> Int4,
        avg_time -> Float4,
        bounce_rate -> Float4,
        known_time_counter -> Int4,
        bounce_counter -> Int4,
        created_at -> Timestamp,
    }
}

joinable!(pageviews -> websites (website_id));
joinable!(stats -> websites (website_id));
joinable!(websites -> users (user_id));

allow_tables_to_appear_in_same_query!(
    pageviews,
    stats,
    users,
    websites,
);
