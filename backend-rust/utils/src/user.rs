use diesel::prelude::*;
use librohan::postgres::schema::users;

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub hashed_pass: &'a str,
}

impl<'a> NewUser<'a> {
    pub fn save(&self, conn: &PgConnection) -> i32 {
        use librohan::postgres::schema::users::{columns as user, table as users};

        diesel::insert_into(users)
            .values(self)
            .returning(user::id)
            .get_result(conn)
            .expect("Failed to insert user")
    }

    pub fn get_id(&self, conn: &PgConnection) -> i32 {
        use librohan::postgres::schema::users::{columns as user, table as users};

        users
            .select(user::id)
            .filter(user::name.eq(self.name))
            .first(conn)
            .expect("Failed to query user")
    }
}

pub const Alice: NewUser = NewUser {
    name: "Alice",
    email: "alice@rohanmarket.com",
    // sha256 hashed string of "alice"
    hashed_pass: "2bd806c97f0e00af1a1fc3328fa763a9269723c8db8fac4f93af71db186d6e90",
};

pub const Bob: NewUser = NewUser {
    name: "Bob",
    email: "bob@rohanmarket.com",
    // sha256 hashed string of "bob"
    hashed_pass: "81b637d8fcd2c6da6359e6963113a1170de795e4b725b84d1e0b4cfd9ec58ce9",
};

pub const Rohan: NewUser = NewUser {
    name: "Rohan",
    email: "rohan@rohanmarket.com",
    // sha256 hashed string of "rohan"
    hashed_pass: "175fe25e87635a040e8e2177232def9001bf05d757cee3848ecc112006426091",
};
