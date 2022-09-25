pub mod models;
pub mod schema;

use std::env;
use diesel::prelude::*;
use dotenvy::dotenv;
use crate::models::{NewUser, User};

pub fn establish_connection() -> MysqlConnection{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut MysqlConnection, name: &str, password: &str) -> User {
    use crate::schema::users;

    let new_user = NewUser { name, password };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new post");

    users::table.order(users::id.desc()).first(conn).unwrap()
}