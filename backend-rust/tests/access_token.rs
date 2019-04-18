#[macro_use]
extern crate diesel;

use diesel::{pg::PgConnection, prelude::*};
use librohan::auth::{authenticate_user, check_token, create_token};
use sha2::Digest;

#[test]
fn test_authenticate_user() {
    let pg_conn = librohan::PgConnectionFactory::new_with_env()
        .establish_connection()
        .unwrap();
    pg_conn.begin_test_transaction().unwrap();

    // Create a new user
    let raw_pass = "test1";
    let hashed_pass = base64::encode(&sha2::Sha256::digest(raw_pass.as_bytes()));
    let new_user = NewUser {
        name: "test 1",
        email: "test1@rohanmarket.com",
        hashed_pass: hashed_pass.as_str(),
    };
    let new_user_id = create_user(&pg_conn, &new_user);

    let should_valid = authenticate_user(&pg_conn, new_user.email, new_user.hashed_pass);
    assert_eq!(should_valid.unwrap(), new_user_id);

    let should_err = authenticate_user(&pg_conn, new_user.email, "hogepass");
    assert!(should_err.is_err());
}

#[test]
fn test_create_token() {
    let redis_conn = librohan::RedisConnectionFactory::new_with_env()
        .establish_connection()
        .unwrap();

    let token = create_token(&redis_conn, 42).unwrap();
    let user_id = check_token(&redis_conn, token.as_str()).unwrap();
    assert_eq!(user_id, 42);

    let res = check_token(&redis_conn, "invalidtoken");
    assert_eq!(res.err().unwrap().name().unwrap(), "invalid token error");
}

use librohan::postgres::schema::users;

#[derive(Debug, Insertable)]
#[table_name = "users"]
struct NewUser<'a> {
    name: &'a str,
    email: &'a str,
    hashed_pass: &'a str,
}

fn create_user(conn: &PgConnection, new_user: &NewUser) -> i32 {
    use librohan::postgres::schema::users::{self, columns};

    diesel::insert_into(users::table)
        .values(new_user)
        .returning(columns::id)
        .get_result(conn)
        .expect("Failed to insert test user")
}
