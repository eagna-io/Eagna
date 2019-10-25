use super::{
    schema::{market_reward_records, user_prize_trade_records, users},
    types::PrizeTradeStatus,
    Postgres,
};
use chrono::{DateTime, Utc};
use diesel::{dsl::sum, prelude::*, result::Error as PgError};
use uuid::Uuid;

pub trait PostgresUserInfra {
    fn save_user<'a>(&self, new_user: NewUser<'a>) -> Result<(), failure::Error>;

    fn query_user(&self, user_id: &str) -> Result<Option<QueryUser>, failure::Error>;

    fn query_user_point(&self, user_id: &str) -> Result<u32, failure::Error>;

    fn save_user_prize_trade_record(
        &self,
        user_id: &str,
        record: NewPrizeTradeRecord,
    ) -> Result<(), failure::Error>;

    fn query_user_prize_trade_records(
        &self,
        user_id: &str,
    ) -> Result<Vec<QueryPrizeTradeRecord>, failure::Error>;
}

pub struct NewUser<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub email: &'a str,
}

pub struct QueryUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

pub struct NewPrizeTradeRecord {
    pub prize_id: Uuid,
    pub point: u32,
    pub time: DateTime<Utc>,
    pub status: PrizeTradeStatus,
}

pub struct QueryPrizeTradeRecord {
    pub point: u32,
    pub time: DateTime<Utc>,
    pub prize_id: Uuid,
    pub status: PrizeTradeStatus,
}

impl PostgresUserInfra for Postgres {
    fn save_user<'a>(&self, new_user: NewUser<'a>) -> Result<(), failure::Error> {
        diesel::insert_into(users::table)
            .values(InsertableUser {
                fb_uid: new_user.id,
                name: new_user.name,
                email: new_user.email,
            })
            .execute(&self.conn)?;
        Ok(())
    }

    fn query_user(&self, user_id: &str) -> Result<Option<QueryUser>, failure::Error> {
        match users::table
            .filter(users::fb_uid.eq(user_id))
            .first::<QueryableUser>(&self.conn)
        {
            Ok(query_res) => Ok(Some(QueryUser {
                id: query_res.fb_uid,
                name: query_res.name,
                email: query_res.email,
                is_admin: query_res.is_admin,
            })),
            Err(PgError::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn query_user_point(&self, user_id: &str) -> Result<u32, failure::Error> {
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
        user_id: &str,
        record: NewPrizeTradeRecord,
    ) -> Result<(), failure::Error> {
        diesel::insert_into(user_prize_trade_records::table)
            .values(InsertablePrizeTradeRecord {
                user_id,
                point: record.point as i32,
                time: record.time,
                prize_id: record.prize_id,
                status: record.status,
            })
            .execute(&self.conn)?;
        Ok(())
    }

    fn query_user_prize_trade_records(
        &self,
        user_id: &str,
    ) -> Result<Vec<QueryPrizeTradeRecord>, failure::Error> {
        Ok(user_prize_trade_records::table
            .filter(user_prize_trade_records::columns::user_id.eq(user_id))
            .select((
                user_prize_trade_records::columns::prize_id,
                user_prize_trade_records::columns::point,
                user_prize_trade_records::columns::time,
                user_prize_trade_records::columns::status,
            ))
            .order(user_prize_trade_records::columns::time.asc())
            .load::<QueryablePrizeTradeRecord>(&self.conn)?
            .into_iter()
            .map(|record| QueryPrizeTradeRecord {
                point: record.point as u32,
                time: record.time,
                prize_id: record.prize_id,
                status: record.status,
            })
            .collect())
    }
}

#[derive(Insertable)]
#[table_name = "users"]
struct InsertableUser<'a> {
    fb_uid: &'a str,
    name: &'a str,
    email: &'a str,
}

#[derive(Queryable)]
struct QueryableUser {
    fb_uid: String,
    name: String,
    email: String,
    is_admin: bool,
    _created: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "user_prize_trade_records"]
struct InsertablePrizeTradeRecord<'a> {
    user_id: &'a str,
    prize_id: Uuid,
    point: i32,
    time: DateTime<Utc>,
    status: PrizeTradeStatus,
}

#[derive(Queryable)]
struct QueryablePrizeTradeRecord {
    prize_id: Uuid,
    point: i32,
    time: DateTime<Utc>,
    status: PrizeTradeStatus,
}
