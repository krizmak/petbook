table! {
    addresses (id) {
        id -> Integer,
        country -> Text,
        state -> Nullable<Text>,
        county -> Nullable<Text>,
        city -> Nullable<Text>,
        postal_code -> Nullable<Text>,
        street -> Nullable<Text>,
        address_line -> Nullable<Text>,
    }
}

table! {
    dogs (id) {
        id -> Integer,
        name -> Text,
        breed -> Integer,
        sex -> Text,
        color -> Integer,
        chip_id -> Nullable<Text>,
        description -> Nullable<Text>,
        birth -> Date,
        death -> Nullable<Date>,
        owner_id -> Integer,
        address_id -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        informal_name -> Nullable<Text>,
        title -> Nullable<Text>,
        email -> Text,
        address_id -> Nullable<Integer>,
        phone -> Nullable<Text>,
        password_hash -> Nullable<Text>,
        google_id -> Nullable<Text>,
        facebook_id -> Nullable<Text>,
        disabled -> Nullable<Bool>,
    }
}

joinable!(dogs -> addresses (address_id));
joinable!(dogs -> users (owner_id));
joinable!(users -> addresses (address_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    dogs,
    users,
);
