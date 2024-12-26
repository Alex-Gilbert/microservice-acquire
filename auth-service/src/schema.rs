// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password_hash -> Bytea,
        password_salt -> Bytea,
    }
}
