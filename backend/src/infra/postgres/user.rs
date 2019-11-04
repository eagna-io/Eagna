use super::{
    schema::{market_reward_records, user_prize_trade_records, users},
    types::PrizeTradeStatus,
    Postgres,
};
use chrono::{DateTime, Utc};
use diesel::{dsl::sum, prelude::*, result::Error as PgError};
use failure::Fallible;
use uuid::Uuid;

pub trait PostgresUserInfra {
    fn save_user<'a>(&self, new_user: NewUser<'a>) -> Result<(), failure::Error>;

    fn query_user(&self, user_id: &Uuid) -> Result<Option<QueryUser>, failure::Error>;

    fn query_user_credentials(&self, email: &str) -> Fallible<Option<QueryUserCredentials>>;

    fn query_user_point(&self, user_id: &Uuid) -> Result<u32, failure::Error>;

    fn save_user_prize_trade_record(
        &self,
        user_id: &Uuid,
        record: NewPrizeTradeRecord,
    ) -> Result<(), failure::Error>;

    fn query_user_prize_trade_records(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<QueryPrizeTradeRecord>, failure::Error>;

    fn query_user_market_reward_records(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<QueryMarketRewardRecord>, failure::Error>;
}

pub struct NewUser<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub email: &'a str,
}

pub struct QueryUser {
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

pub struct QueryUserCredentials {
    pub id: Uuid,
    pub cred: Vec<u8>,
    pub salt: Vec<u8>,
}

pub struct NewPrizeTradeRecord {
    pub id: Uuid,
    pub prize_id: Uuid,
    pub point: u32,
    pub time: DateTime<Utc>,
}

pub struct QueryPrizeTradeRecord {
    pub id: Uuid,
    pub point: u32,
    pub time: DateTime<Utc>,
    pub prize_id: Uuid,
    pub status: PrizeTradeStatus,
    pub processed_at: Option<DateTime<Utc>>,
}

pub struct QueryMarketRewardRecord {
    pub market_id: Uuid,
    pub point: u32,
}

impl PostgresUserInfra for Postgres {
    fn save_user<'a>(&self, new_user: NewUser<'a>) -> Result<(), failure::Error> {
        diesel::insert_into(users::table)
            .values(InsertableUser {
                id: new_user.id,
                name: new_user.name,
                email: new_user.email,
            })
            .execute(&self.conn)?;
        Ok(())
    }

    fn query_user(&self, user_id: &Uuid) -> Result<Option<QueryUser>, failure::Error> {
        match users::table
            .filter(users::id.eq(user_id))
            .select((users::name, users::email, users::is_admin))
            .first::<QueryableUser>(&self.conn)
        {
            Ok(query_res) => Ok(Some(QueryUser {
                name: query_res.name,
                email: query_res.email,
                is_admin: query_res.is_admin,
            })),
            Err(PgError::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn query_user_credentials(&self, email: &str) -> Fallible<Option<QueryUserCredentials>> {
        match users::table
            .filter(users::email.eq(email))
            .select((users::id, users::credential, users::salt))
            .first::<(Uuid, Vec<u8>, Vec<u8>)>(&self.conn)
        {
            Ok((id, cred, salt)) => Ok(Some(QueryUserCredentials { id, cred, salt })),
            Err(PgError::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn query_user_point(&self, user_id: &Uuid) -> Result<u32, failure::Error> {
        let reward_points = market_reward_records::table
            .filter(market_reward_records::user_id.eq(user_id))
            .select(sum(market_reward_records::point))
            .first::<Option<i64>>(&self.conn)?
            .unwrap_or(0);
        let used_points = user_prize_trade_records::table
            .filter(user_prize_trade_records::user_id.eq(user_id))
            .select(sum(user_prize_trade_records::point))
            .first::<Option<i64>>(&self.conn)?
            .unwrap_or(0);
        let user_points = reward_points + used_points;
        assert!(user_points >= 0);
        Ok(user_points as u32)
    }

    fn save_user_prize_trade_record(
        &self,
        user_id: &Uuid,
        record: NewPrizeTradeRecord,
    ) -> Result<(), failure::Error> {
        diesel::insert_into(user_prize_trade_records::table)
            .values(InsertablePrizeTradeRecord {
                id: record.id,
                user_id: *user_id,
                point: record.point as i32,
                time: record.time,
                prize_id: record.prize_id,
                status: PrizeTradeStatus::Requested,
            })
            .execute(&self.conn)?;
        Ok(())
    }

    fn query_user_prize_trade_records(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<QueryPrizeTradeRecord>, failure::Error> {
        Ok(user_prize_trade_records::table
            .filter(user_prize_trade_records::columns::user_id.eq(user_id))
            .select((
                user_prize_trade_records::id,
                user_prize_trade_records::prize_id,
                user_prize_trade_records::point,
                user_prize_trade_records::time,
                user_prize_trade_records::status,
                user_prize_trade_records::processed_at,
            ))
            .order(user_prize_trade_records::columns::time.asc())
            .load::<QueryablePrizeTradeRecord>(&self.conn)?
            .into_iter()
            .map(|record| QueryPrizeTradeRecord {
                id: record.id,
                point: record.point as u32,
                time: record.time,
                prize_id: record.prize_id,
                status: record.status,
                processed_at: record.processed_at,
            })
            .collect())
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
}

#[derive(Insertable)]
#[table_name = "users"]
struct InsertableUser<'a> {
    id: Uuid,
    name: &'a str,
    email: &'a str,
}

#[derive(Queryable)]
struct QueryableUser {
    name: String,
    email: String,
    is_admin: bool,
}

#[derive(Insertable)]
#[table_name = "user_prize_trade_records"]
struct InsertablePrizeTradeRecord {
    id: Uuid,
    user_id: Uuid,
    prize_id: Uuid,
    point: i32,
    time: DateTime<Utc>,
    status: PrizeTradeStatus,
}

#[derive(Queryable)]
struct QueryablePrizeTradeRecord {
    id: Uuid,
    prize_id: Uuid,
    point: i32,
    time: DateTime<Utc>,
    status: PrizeTradeStatus,
    processed_at: Option<DateTime<Utc>>,
}

#[derive(Queryable)]
struct QueryableMarketRewardRecord {
    market_id: Uuid,
    point: i32,
}
