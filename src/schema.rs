// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
