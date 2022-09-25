use self::models::*;
use diesel::prelude::*;
use fake::{Fake};
use fake::locales::EN;
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
        println!("{}", user.id);
        println!("-----------\n");
        println!("{}", user.name);
    }

    use fake::faker::name::raw::*;

    let new_name: String = Name(EN).fake();
    let new_passwords: String = Name(EN).fake();

    let user = create_user(connection, &new_name, &new_passwords);
    println!("\nSaved draft {} with id {}", user.name, user.id);
}