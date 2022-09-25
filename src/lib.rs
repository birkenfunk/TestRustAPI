pub mod models;
pub mod schema;

use std::env;
use diesel::prelude::*;
use dotenvy::dotenv;
use crate::models::{NewUser, User};
use self::schema::users::dsl::*;

pub fn establish_connection() -> MysqlConnection{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut MysqlConnection, new_name: &str, new_password: &str) -> User {
    use crate::schema::users;

    let new_user = NewUser { name: new_name, password: new_password };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new post");

    users::table.order(users::id.desc()).first(conn).unwrap()
}

pub fn get_all_users(conn: &mut MysqlConnection) -> Vec<User> {
    let mut all_users: Vec<User> = users
        .load::<User>(conn)
        .expect("Error loading users");
    all_users.sort_by_key(|d| d.id);
    all_users
}

pub fn get_user_by_name(search_name: &str, conn: &mut MysqlConnection) -> Option<User> {
    let all_users: Vec<User> = users
        .filter(name.eq(search_name))
        .limit(1)
        .load::<User>(conn)
        .expect("Error loading users");
     all_users.first().cloned()
}
