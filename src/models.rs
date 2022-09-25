use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub user_password: String,
    pub account_deactivated: bool,
}