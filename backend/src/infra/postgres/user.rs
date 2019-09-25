use super::{
    schema::{user_prize_trade_history, user_reward_point_history, users},
    types::PrizeTradeStatus,
    Postgres,
};
use chrono::{DateTime, Utc};
use diesel::{dsl::sum, prelude::*, result::Error as PgError};
use std::cmp::Ordering;
use uuid::Uuid;

pub trait PostgresUserInfra {
    fn save_user<'a>(&self, new_user: NewUser<'a>) -> Result<(), failure::Error>;

    fn query_user(&self, user_id: &str) -> Result<Option<QueryUser>, failure::Error>;

    fn query_user_point(&self, user_id: &str) -> Result<u32, failure::Error>;

    fn query_user_point_history(
        &self,
        user_id: &str,
    ) -> Result<Vec<PointHistoryItem>, failure::Error>;
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

pub enum PointHistoryItem {
    MarketReward {
        amount: u32,
        time: DateTime<Utc>,
        market_id: Uuid,
    },
    PrizeTrade {
        price: u32,
        time: DateTime<Utc>,
        prize_id: Uuid,
        status: PrizeTradeStatus,
    },
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

    /// Userの現在の保有コインを取得する。
    /// Userが存在しない場合は0を返す。
    fn query_user_point(&self, user_id: &str) -> Result<u32, failure::Error> {
        let earned = user_reward_point_history::table
            .filter(user_reward_point_history::columns::user_id.eq(user_id))
            .select(sum(user_reward_point_history::columns::point))
            .first::<Option<i64>>(&self.conn)?
            .unwrap_or(0);
        let consumed = user_prize_trade_history::table
            .filter(user_prize_trade_history::columns::user_id.eq(user_id))
            .select(sum(user_prize_trade_history::columns::price))
            .first::<Option<i64>>(&self.conn)?
            .unwrap_or(0);
        assert!(earned >= consumed);
        return Ok((earned - consumed) as u32);
    }

    fn query_user_point_history(
        &self,
        user_id: &str,
    ) -> Result<Vec<PointHistoryItem>, failure::Error> {
        let mut reward_history = user_reward_point_history::table
            .filter(user_reward_point_history::columns::user_id.eq(user_id))
            .select((
                user_reward_point_history::columns::market_id,
                user_reward_point_history::columns::point,
                user_reward_point_history::columns::time,
            ))
            .order(user_reward_point_history::columns::time.asc())
            .load::<QueryableMarketRewardHistoryItem>(&self.conn)?;
        let mut trade_history = user_prize_trade_history::table
            .filter(user_prize_trade_history::columns::user_id.eq(user_id))
            .select((
                user_prize_trade_history::columns::prize_id,
                user_prize_trade_history::columns::price,
                user_prize_trade_history::columns::time,
                user_prize_trade_history::columns::status,
            ))
            .order(user_prize_trade_history::columns::time.asc())
            .load::<QueryablePrizeTradeHistoryItem>(&self.conn)?;

        // time が小さいものから順に取り出していく
        let mut history = Vec::with_capacity(reward_history.len() + trade_history.len());
        loop {
            let which = match (reward_history.first(), trade_history.first()) {
                (Some(reward_item), Some(trade_item)) => reward_item.time.cmp(&trade_item.time),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => break,
            };
            let item = match which {
                Ordering::Less | Ordering::Equal => {
                    let reward_item = reward_history.pop().unwrap();
                    PointHistoryItem::MarketReward {
                        amount: reward_item.point as u32,
                        market_id: reward_item.market_id,
                        time: reward_item.time,
                    }
                }
                Ordering::Greater => {
                    let trade_item = trade_history.pop().unwrap();
                    PointHistoryItem::PrizeTrade {
                        price: trade_item.price as u32,
                        prize_id: trade_item.prize_id,
                        time: trade_item.time,
                        status: trade_item.status,
                    }
                }
            };
            history.push(item);
        }
        Ok(history)
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

#[derive(Queryable)]
struct QueryableMarketRewardHistoryItem {
    market_id: Uuid,
    point: i32,
    time: DateTime<Utc>,
}

#[derive(Queryable)]
struct QueryablePrizeTradeHistoryItem {
    prize_id: Uuid,
    price: i32,
    time: DateTime<Utc>,
    status: PrizeTradeStatus,
}
