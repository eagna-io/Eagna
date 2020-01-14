use super::{schema::users, Postgres};
use diesel::{prelude::*, result::Error as PgError};
use uuid::Uuid;

pub trait PostgresUserInfra {
    fn save_user<'a>(&self, new_user: NewUser<'a>) -> anyhow::Result<()>;

    fn query_user(&self, user_id: &Uuid) -> anyhow::Result<Option<QueryUser>>;

    fn query_user_credentials(&self, email: &str) -> anyhow::Result<Option<QueryUserCredentials>>;

    fn update_user_coin(&self, user_id: &Uuid, coin: u32) -> anyhow::Result<()>;

    /// point += point_delta
    fn update_assign_user_point(&self, user_id: &Uuid, point_delta: i32) -> anyhow::Result<()>;
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub email: &'a str,
    pub credential: &'a [u8],
    pub salt: &'a [u8],
}

#[derive(Queryable)]
pub struct QueryUser {
    pub name: String,
    pub email: String,
    pub coin: i32,  // >= 0
    pub point: i32, // >= 0
    pub is_admin: bool,
}

#[derive(Queryable)]
pub struct QueryUserCredentials {
    pub id: Uuid,
    pub cred: Vec<u8>,
    pub salt: Vec<u8>,
}

pub struct QueryMarketRewardRecord {
    pub market_id: Uuid,
    pub point: u32,
}

impl PostgresUserInfra for Postgres {
    fn save_user<'a>(&self, new_user: NewUser<'a>) -> anyhow::Result<()> {
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(&self.conn)?;
        Ok(())
    }

    fn query_user(&self, user_id: &Uuid) -> anyhow::Result<Option<QueryUser>> {
        match users::table
            .filter(users::id.eq(user_id))
            .select((
                users::name,
                users::email,
                users::coin,
                users::point,
                users::is_admin,
            ))
            .first::<QueryUser>(&self.conn)
        {
            Ok(user) => Ok(Some(user)),
            Err(PgError::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn query_user_credentials(&self, email: &str) -> anyhow::Result<Option<QueryUserCredentials>> {
        match users::table
            .filter(users::email.eq(email))
            .select((users::id, users::credential, users::salt))
            .first::<QueryUserCredentials>(&self.conn)
        {
            Ok(creds) => Ok(Some(creds)),
            Err(PgError::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn update_user_coin(&self, user_id: &Uuid, coin: u32) -> anyhow::Result<()> {
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(users::coin.eq(coin as i32))
            .execute(&self.conn)?;
        Ok(())
    }

    fn update_assign_user_point(&self, user_id: &Uuid, point_delta: i32) -> anyhow::Result<()> {
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(users::coin.eq(users::coin + point_delta))
            .execute(&self.conn)?;
        Ok(())
    }
}
