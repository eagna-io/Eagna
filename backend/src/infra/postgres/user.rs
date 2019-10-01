use super::{
    schema::{user_prize_trade_history, user_reward_point_history, users},
    types::PrizeTradeStatus,
    Postgres,
};
use chrono::{DateTime, Utc};
use diesel::{prelude::*, result::Error as PgError};
use std::cmp::Ordering;
use uuid::Uuid;

pub trait PostgresUserInfra {
    fn save_user<'a>(&self, new_user: NewUser<'a>) -> Result<(), failure::Error>;

    fn query_user(&self, user_id: &str) -> Result<Option<QueryUser>, failure::Error>;

    fn save_user_point_history(
        &self,
        user_id: &str,
        history: NewPointHistoryItem,
    ) -> Result<(), failure::Error>;

    fn query_user_point_history(
        &self,
        user_id: &str,
    ) -> Result<Vec<QueryPointHistoryItem>, failure::Error>;
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

pub enum NewPointHistoryItem {
    MarketReward(NewMarketRewardHistoryItem),
    PrizeTrade(NewPrizeTradeHistoryItem),
}

pub struct NewMarketRewardHistoryItem {
    pub point: u32,
    pub time: DateTime<Utc>,
    pub market_id: Uuid,
}

pub struct NewPrizeTradeHistoryItem {
    pub point: u32,
    pub time: DateTime<Utc>,
    pub prize_id: Uuid,
    pub status: PrizeTradeStatus,
}

pub enum QueryPointHistoryItem {
    MarketReward(QueryMarketRewardHistoryItem),
    PrizeTrade(QueryPrizeTradeHistoryItem),
}

pub struct QueryMarketRewardHistoryItem {
    pub point: u32,
    pub time: DateTime<Utc>,
    pub market_id: Uuid,
}

pub struct QueryPrizeTradeHistoryItem {
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

    fn save_user_point_history(
        &self,
        user_id: &str,
        item: NewPointHistoryItem,
    ) -> Result<(), failure::Error> {
        match item {
            NewPointHistoryItem::MarketReward(item) => {
                diesel::insert_into(user_reward_point_history::table)
                    .values(InsertableMarketRewardHistoryItem {
                        user_id,
                        point: item.point as i32,
                        time: item.time,
                        market_id: item.market_id,
                    })
                    .execute(&self.conn)?;
            }
            NewPointHistoryItem::PrizeTrade(item) => {
                diesel::insert_into(user_prize_trade_history::table)
                    .values(InsertablePrizeTradeHistoryItem {
                        user_id,
                        point: item.point as i32,
                        time: item.time,
                        prize_id: item.prize_id,
                        status: item.status,
                    })
                    .execute(&self.conn)?;
            }
        }
        Ok(())
    }

    fn query_user_point_history(
        &self,
        user_id: &str,
    ) -> Result<Vec<QueryPointHistoryItem>, failure::Error> {
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
                user_prize_trade_history::columns::point,
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
                    // reward_history から取り出す
                    let infra_reward_item = reward_history.pop().unwrap();
                    let reward_item = QueryMarketRewardHistoryItem {
                        point: infra_reward_item.point as u32,
                        market_id: infra_reward_item.market_id,
                        time: infra_reward_item.time,
                    };
                    QueryPointHistoryItem::MarketReward(reward_item)
                }
                Ordering::Greater => {
                    // trade_history から取り出す
                    let infra_trade_item = trade_history.pop().unwrap();
                    let trade_item = QueryPrizeTradeHistoryItem {
                        point: infra_trade_item.point as u32,
                        time: infra_trade_item.time,
                        prize_id: infra_trade_item.prize_id,
                        status: infra_trade_item.status,
                    };
                    QueryPointHistoryItem::PrizeTrade(trade_item)
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

#[derive(Insertable)]
#[table_name = "user_reward_point_history"]
struct InsertableMarketRewardHistoryItem<'a> {
    user_id: &'a str,
    market_id: Uuid,
    point: i32,
    time: DateTime<Utc>,
}

#[derive(Queryable)]
struct QueryableMarketRewardHistoryItem {
    market_id: Uuid,
    point: i32,
    time: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "user_prize_trade_history"]
struct InsertablePrizeTradeHistoryItem<'a> {
    user_id: &'a str,
    prize_id: Uuid,
    point: i32,
    time: DateTime<Utc>,
    status: PrizeTradeStatus,
}

#[derive(Queryable)]
struct QueryablePrizeTradeHistoryItem {
    prize_id: Uuid,
    point: i32,
    time: DateTime<Utc>,
    status: PrizeTradeStatus,
}
