#[macro_use]
extern crate diesel;

use diesel::Connection;

use librohan::auth::{authenticate_user, check_token, create_token};

#[test]
fn test_authenticate_user() {
    let pg_conn = librohan::PgConnectionFactory::new_with_env()
        .establish_connection()
        .unwrap();
    pg_conn.begin_test_transaction().unwrap();

    // Create a new user
    let new_user = utils::NewUser {
        name: "test 1",
        email: "test1@rohanmarket.com",
        hashed_pass: "testhashedpassword",
    };
    let new_user_id = utils::create_user(&pg_conn, &new_user);

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
