use fake::{Fake};
use fake::locales::EN;
use first_rust_project::*;


fn main() {
    let connection = &mut establish_connection();
    let results = get_all_users(connection);

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

    let search_name = "Alex1";

    let option_user = get_user_by_name(search_name, connection);

    if option_user.is_some() {
        println!("Found User {} with Id {}", search_name, user.id);
    }else {
        println!("User {} not found", search_name);
    }
}