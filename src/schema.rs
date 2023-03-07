// @generated automatically by Diesel CLI.

diesel::table! {
    breach_data (id) {
        id -> Integer,
        date_reported -> Timestamp,
        organization_name -> Text,
        date_of_breach -> Nullable<Timestamp>,
        affected_count -> Nullable<Integer>,
        loc -> Integer,
        link -> Nullable<Text>,
        breach_type -> Integer,
        affected_count_local -> Nullable<Integer>,
    }
}

diesel::table! {
    classification (id) {
        id -> Integer,
        breach_data_id -> Integer,
        content -> Text,
        classification_type -> Integer,
    }
}

diesel::table! {
    last_retrieved (id) {
        id -> Integer,
        loc -> Integer,
        retrieved_date -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    breach_data,
    classification,
    last_retrieved,
);
