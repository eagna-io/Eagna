use crate::domain::models::user::{User, UserId};
use diesel::{pg::PgConnection, prelude::*, result::Error as PgError};

pub fn query_user(conn: &PgConnection, user_id: &UserId) -> Result<Option<User>, PgError> {
    use crate::infra::postgres::schema::users;

    match users::table
        .select((users::id, users::name, users::email))
        .filter(users::id.eq(user_id.0))
        .first(conn)
    {
        Ok((id, name, email)) => Ok(Some(User {
            id: UserId(id),
            name,
            email,
        })),
        Err(PgError::NotFound) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn query_user_by_email_and_hashed_pass(
    conn: &PgConnection,
    email: &str,
    hashed_pass: &str,
) -> Result<Option<User>, PgError> {
    use crate::infra::postgres::schema::users;

    match users::table
        .select((users::id, users::name, users::email))
        .filter(users::email.eq(email))
        .filter(users::hashed_pass.eq(hashed_pass))
        .first(conn)
    {
        Ok((id, name, email)) => Ok(Some(User {
            id: UserId(id),
            name,
            email,
        })),
        Err(PgError::NotFound) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn query_all_user_ids(conn: &PgConnection) -> Result<Vec<UserId>, PgError> {
    use crate::infra::postgres::schema::users;

    users::table
        .select(users::id)
        .load::<i32>(conn)
        .map(|ids| ids.into_iter().map(|id| UserId(id)).collect())
}
