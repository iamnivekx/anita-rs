// @generated automatically by Diesel CLI.

diesel::table! {
    keys (id) {
        id -> Int4,
        chain -> Varchar,
        secret -> Varchar,
        pubkey -> Varchar,
        address -> Varchar,
        suffix -> Varchar,
        used_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}
