table! {
    dogs (id) {
        id -> Integer,
        name -> Text,
        age -> Integer,
        bought_at -> Nullable<Timestamp>,
        author_id -> Integer,
    }
}

table! {
    userauth (id) {
        id -> Integer,
        user_id -> Integer,
        password_hash -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        age -> Integer,
    }
}

joinable!(dogs -> users (author_id));
joinable!(userauth -> users (user_id));

allow_tables_to_appear_in_same_query!(
    dogs,
    userauth,
    users,
);
