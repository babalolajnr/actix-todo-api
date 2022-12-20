// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
        done -> Bool,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

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

diesel::joinable!(todo -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    todo,
    user,
);
