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
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        age -> Integer,
    }
}

joinable!(dogs -> users (author_id));

allow_tables_to_appear_in_same_query!(
    dogs,
    users,
);
