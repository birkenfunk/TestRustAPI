// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        password -> Varchar,
        deactivated -> Bool,
    }
}
