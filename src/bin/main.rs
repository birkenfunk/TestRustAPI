use self::models::*;
use diesel::prelude::*;
use first_rust_project::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .limit(5)
        .load::<User>(connection)
        .expect("Error loading users");

    println!("Displaying {} posts", results.len());
    for user in results {
        println!("{}", user.user_id);
        println!("-----------\n");
        println!("{}", user.user_name);
    }
}