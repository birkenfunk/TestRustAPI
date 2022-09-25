// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        user_name -> Varchar,
        user_password -> Varchar,
        account_deactivated -> Bool,
    }
}
