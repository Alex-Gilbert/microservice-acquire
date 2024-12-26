pub mod schema;

use argon2::{
    password_hash::{rand_core, SaltString},
    Argon2,
};
use diesel::{Connection, PgConnection};

use diesel::prelude::*;
use schema::users;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct User {
    id: i32,
    username: String,
    password_hash: Vec<u8>,
    password_salt: Vec<u8>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    username: String,
    password_hash: Vec<u8>,
    password_salt: Vec<u8>,
}

fn register(connection: &mut PgConnection, new_username: &str, new_password: &str) {
    use schema::users::dsl::*;

    let new_password = new_password.as_bytes();
    let salt = SaltString::generate(&mut rand_core::OsRng);

    let mut output_key_material = [0u8; 32];

    let _ = Argon2::default().hash_password_into(
        new_password,
        salt.as_str().as_bytes(),
        &mut output_key_material,
    );

    let new_user = NewUser {
        username: new_username.to_string(),
        password_hash: output_key_material.to_vec(),
        password_salt: salt.as_str().as_bytes().to_vec(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(connection)
        .expect("Error saving new user");
}

pub fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    //prompt user if they want to register or login
    println!("Would you like to (r)egister or (l)ogin or (v)iew users?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    match input {
        "r" => {
            println!("Enter a username:");
            let mut new_username = String::new();
            std::io::stdin().read_line(&mut new_username).unwrap();
            let new_username = new_username.trim();

            println!("Enter a password:");
            let mut new_password = String::new();
            std::io::stdin().read_line(&mut new_password).unwrap();
            let new_password = new_password.trim();

            let connection = &mut establish_connection();
            register(connection, &new_username, &new_password);
        }
        "l" => {
            println!("Not implemented");
        }
        "v" => {
            use schema::users::dsl::*;

            let connection = &mut establish_connection();

            let results = users
                .limit(5)
                .select(User::as_select())
                .load(connection)
                .expect("Error loading users");

            println!("Displaying {} users", results.len());
            for user in results {
                println!("{:?}", user);
            }
        }
        _ => {
            println!("Not implemented");
        }
    }
}
