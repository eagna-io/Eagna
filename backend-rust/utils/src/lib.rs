#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use librohan::postgres::schema::users;

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub hashed_pass: &'a str,
}

pub fn create_user(conn: &PgConnection, new_user: &NewUser) -> i32 {
    use librohan::postgres::schema::users::{self, columns};

    diesel::insert_into(users::table)
        .values(new_user)
        .returning(columns::id)
        .get_result(conn)
        .expect("Failed to insert test user")
}
