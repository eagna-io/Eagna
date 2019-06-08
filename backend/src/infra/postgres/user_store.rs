use crate::domain::{
    models::user::{User, UserId},
    services::user_store::NewUser,
};
use diesel::{pg::PgConnection, prelude::*, result::Error as PgError};

pub fn query_user(conn: &PgConnection, user_id: &UserId) -> Result<Option<User>, PgError> {
    use crate::infra::postgres::schema::users;

    match users::table
        .select((users::id, users::name, users::email, users::is_admin))
        .filter(users::id.eq(user_id.as_str()))
        .first::<(String, String, String, bool)>(conn)
    {
        Ok((id, name, email, is_admin)) => Ok(Some(User {
            id: UserId::from_str(id.as_str()),
            name,
            email,
            is_admin,
        })),
        Err(PgError::NotFound) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn query_all_user_ids(conn: &PgConnection) -> Result<Vec<UserId>, PgError> {
    use crate::infra::postgres::schema::users;

    users::table
        .select(users::id)
        .load::<String>(conn)
        .map(|ids| ids.iter().map(|id| UserId::from_str(id)).collect())
}

pub fn save_user<'a>(conn: &PgConnection, new_user: NewUser<'a>) -> Result<(), PgError> {
    use crate::infra::postgres::schema::users;

    #[derive(Insertable)]
    #[table_name = "users"]
    struct InnerNewUser<'b> {
        id: &'b str,
        name: &'b str,
        email: &'b str,
    }

    diesel::insert_into(users::table)
        .values(InnerNewUser {
            id: new_user.id.as_str(),
            name: new_user.name,
            email: new_user.email,
        })
        .execute(conn)
        .map(|_| ())
}
