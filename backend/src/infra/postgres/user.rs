use super::{
    schema::{market_reward_records, users},
    Postgres,
};
use diesel::{prelude::*, result::Error as PgError};
use failure::Fallible;
use uuid::Uuid;

pub trait PostgresUserInfra {
    fn save_user<'a>(&self, new_user: NewUser<'a>) -> Result<(), failure::Error>;

    fn query_user(&self, user_id: &Uuid) -> Result<Option<QueryUser>, failure::Error>;

    fn query_user_credentials(&self, email: &str) -> Fallible<Option<QueryUserCredentials>>;

    fn query_user_market_reward_records(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<QueryMarketRewardRecord>, failure::Error>;

    fn update_user_coin(&self, user_id: &Uuid, coin: u32) -> Fallible<()>;
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
    fn save_user<'a>(&self, new_user: NewUser<'a>) -> Result<(), failure::Error> {
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(&self.conn)?;
        Ok(())
    }

    fn query_user(&self, user_id: &Uuid) -> Result<Option<QueryUser>, failure::Error> {
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

    fn query_user_credentials(&self, email: &str) -> Fallible<Option<QueryUserCredentials>> {
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

    fn query_user_market_reward_records(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<QueryMarketRewardRecord>, failure::Error> {
        Ok(market_reward_records::table
            .filter(market_reward_records::user_id.eq(user_id))
            .select((
                market_reward_records::market_id,
                market_reward_records::point,
            ))
            .load::<QueryableMarketRewardRecord>(&self.conn)?
            .into_iter()
            .map(|record| QueryMarketRewardRecord {
                market_id: record.market_id,
                point: record.point as u32,
            })
            .collect())
    }

    fn update_user_coin(&self, user_id: &Uuid, coin: u32) -> Fallible<()> {
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(users::coin.eq(coin as i32))
            .execute(&self.conn)?;
        Ok(())
    }
}

#[derive(Queryable)]
struct QueryableMarketRewardRecord {
    market_id: Uuid,
    point: i32,
}
