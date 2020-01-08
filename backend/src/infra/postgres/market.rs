use super::{
    types::{MarketStatus, OrderType},
    Postgres,
};
use chrono::{DateTime, Utc};
use diesel::{dsl::now, pg::expression::dsl::any, prelude::*};
use uuid::Uuid;

/*
 * =================
 * Interface
 * =================
 */

pub trait PostgresMarketInfra {
    // 対象のマーケットに対する更新をトランザクションの期間ロックする。
    // Postgres の FOR UPDATE ロックを使用する想定
    fn lock_market(&self, market_id: &Uuid) -> Result<(), failure::Error>;

    fn insert_upcoming_market<'a>(&self, market: NewMarket<'a>) -> Result<(), failure::Error>;

    fn update_market_status(
        &self,
        market_id: &Uuid,
        market_status: &MarketStatus,
    ) -> Result<(), failure::Error>;

    /// - market_statusをResolvedに変更
    /// - resolved_token_name を設定
    /// - resolved_at を設定
    fn resolve_market(
        &self,
        market_id: &Uuid,
        resolved_token_name: &str,
    ) -> Result<(), failure::Error>;

    fn insert_orders<'a>(
        &self,
        market_id: &'a Uuid,
        orders: &'a mut dyn Iterator<Item = NewOrder<'a>>,
    ) -> Result<(), failure::Error>;

    fn insert_reward_records<'a>(
        &self,
        market_id: Uuid,
        records: &'a mut dyn Iterator<Item = NewRewardRecord>,
    ) -> Result<(), failure::Error>;

    fn query_market_by_id(&self, id: &Uuid) -> Result<Option<QueryMarket>, failure::Error>;

    /// 時系列順にソートされた `QueryOrder` を返す。
    /// 古いものが最初に、新しいものが最後に来る
    fn query_orders_by_market_id(
        &self,
        market_ids: &Uuid,
    ) -> Result<Vec<QueryOrder>, failure::Error>;

    fn query_market_ids_by_status(
        &self,
        status: &[MarketStatus],
    ) -> Result<Vec<Uuid>, failure::Error>;

    fn query_market_ids_participated_by_user(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<Uuid>, failure::Error>;

    fn query_market_ids_ready_to_open(&self) -> Result<Vec<Uuid>, failure::Error>;

    fn query_market_ids_ready_to_close(&self) -> Result<Vec<Uuid>, failure::Error>;
}

pub struct NewMarket<'a> {
    pub id: &'a Uuid,
    pub title: &'a str,
    pub organizer_id: &'a Uuid,
    pub description: &'a str,
    pub lmsr_b: i32,
    pub open: &'a DateTime<Utc>,
    pub close: &'a DateTime<Utc>,
    // tokenのidxカラムは、この順序で設定される。
    pub tokens: &'a mut dyn Iterator<Item = NewToken<'a>>,
    // status は常にUpcoming
}

pub struct NewToken<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub thumbnail_url: &'a str,
    pub idx: i32,
}

pub struct NewOrder<'a> {
    pub local_id: i32,
    pub user_id: Uuid,
    pub token_name: Option<&'a str>,
    pub amount_token: i32,
    pub amount_coin: i32,
    pub type_: OrderType,
    pub time: DateTime<Utc>,
}

pub struct NewRewardRecord {
    pub user_id: Uuid,
    pub point: i32,
}

#[derive(Debug, Clone)]
pub struct QueryMarket {
    pub id: Uuid,
    pub title: String,
    pub organizer_id: Uuid,
    pub description: String,
    pub lmsr_b: i32,
    pub open: DateTime<Utc>,
    pub close: DateTime<Utc>,
    pub status: MarketStatus,
    // tokenのidxカラム順にソートされている
    pub tokens: Vec<QueryToken>,
    pub resolved_token_name: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub reward_records: Option<Vec<QueryRewardRecord>>,
}

#[derive(Debug, Clone)]
pub struct QueryToken {
    pub name: String,
    pub description: String,
    pub thumbnail_url: String,
}

#[derive(Debug, Clone)]
pub struct QueryRewardRecord {
    pub user_id: Uuid,
    pub point: u32,
}

#[derive(Debug, Clone)]
pub struct QueryOrder {
    pub local_id: i32,
    pub user_id: Uuid,
    pub market_id: Uuid,
    pub token_name: Option<String>,
    pub amount_token: i32,
    pub amount_coin: i32,
    pub type_: OrderType,
    pub time: DateTime<Utc>,
}

/*
 * ==================
 * Implementation
 * ==================
 */

use super::schema::{market_reward_records, market_tokens, markets, orders};

impl PostgresMarketInfra for Postgres {
    fn lock_market(&self, market_id: &Uuid) -> Result<(), failure::Error> {
        markets::table
            .select(markets::columns::id)
            .filter(markets::columns::id.eq(market_id))
            .for_update()
            .first::<Uuid>(&self.conn)?;
        Ok(())
    }

    fn insert_upcoming_market<'a>(&self, market: NewMarket<'a>) -> Result<(), failure::Error> {
        // Insert market
        let insert_market = InsertableMarket {
            id: market.id,
            title: market.title,
            organizer_id: market.organizer_id,
            description: market.description,
            lmsr_b: market.lmsr_b,
            open: market.open,
            close: market.close,
        };
        diesel::insert_into(markets::table)
            .values(insert_market)
            .execute(&self.conn)?;

        let market_id = market.id.clone();

        // Insert tokens
        let insert_tokens: Vec<_> = market
            .tokens
            .map(|token| InsertableToken {
                name: token.name,
                description: token.description,
                thumbnail_url: token.thumbnail_url,
                market_id: &market_id,
                idx: token.idx,
            })
            .collect();
        diesel::insert_into(market_tokens::table)
            .values(insert_tokens)
            .execute(&self.conn)?;

        Ok(())
    }

    fn update_market_status(
        &self,
        market_id: &Uuid,
        status: &MarketStatus,
    ) -> Result<(), failure::Error> {
        diesel::update(markets::table.filter(markets::id.eq(market_id)))
            .set(markets::status.eq(status))
            .execute(&self.conn)?;
        Ok(())
    }

    fn resolve_market(
        &self,
        market_id: &Uuid,
        resolved_token_name: &str,
    ) -> Result<(), failure::Error> {
        diesel::update(markets::table.filter(markets::id.eq(market_id)))
            .set((
                markets::status.eq(MarketStatus::Resolved),
                markets::resolved_token_name.eq(resolved_token_name),
                markets::resolved_at.eq(Utc::now()),
            ))
            .execute(&self.conn)?;
        Ok(())
    }

    fn insert_orders<'a>(
        &self,
        market_id: &'a Uuid,
        orders: &'a mut dyn Iterator<Item = NewOrder<'a>>,
    ) -> Result<(), failure::Error> {
        let insert_orders: Vec<InsertableOrder> = orders
            .map(|order| InsertableOrder {
                market_local_id: order.local_id,
                user_id: order.user_id,
                token_name: order.token_name,
                amount_token: order.amount_token,
                amount_coin: order.amount_coin,
                time: order.time,
                type_: order.type_,
                market_id: *market_id,
            })
            .collect();
        diesel::insert_into(orders::table)
            .values(&insert_orders)
            .execute(&self.conn)?;
        Ok(())
    }

    fn insert_reward_records<'a>(
        &self,
        market_id: Uuid,
        records: &'a mut dyn Iterator<Item = NewRewardRecord>,
    ) -> Result<(), failure::Error> {
        let insert_records = records
            .map(|record| InsertableRewardRecord {
                market_id,
                user_id: record.user_id,
                point: record.point,
            })
            .collect::<Vec<_>>();
        diesel::insert_into(market_reward_records::table)
            .values(&insert_records)
            .execute(&self.conn)?;
        Ok(())
    }

    // # NOTE
    // tokens は market_tokens テーブルの idx カラムの順に取り出される。
    fn query_market_by_id(&self, id: &Uuid) -> Result<Option<QueryMarket>, failure::Error> {
        // QueryableMarket を取得
        let raw_market = match markets::table
            .find(id)
            .first::<QueryableMarket>(&self.conn)
            .optional()?
        {
            None => return Ok(None),
            Some(m) => m,
        };

        // idxカラムの順にQueryableTokenを取得
        let raw_market_tokens = market_tokens::table
            .filter(market_tokens::columns::market_id.eq(id))
            .order(market_tokens::columns::idx.asc())
            .load::<QueryableToken>(&self.conn)?;

        // statusがresolvedのとき、QueryableRewardRecordsを取得
        let reward_records = if raw_market.status == MarketStatus::Resolved {
            Some(
                market_reward_records::table
                    .filter(market_reward_records::columns::market_id.eq(id))
                    .load::<QueryableRewardRecords>(&self.conn)?,
            )
        } else {
            None
        };

        let market = QueryMarket::from_parts(raw_market, raw_market_tokens, reward_records);

        Ok(Some(market))
    }

    /// 時系列順にソートされた `QueryOrder` を返す。
    /// 古いものが最初に、新しいものが最後に来る
    fn query_orders_by_market_id(
        &self,
        market_id: &Uuid,
    ) -> Result<Vec<QueryOrder>, failure::Error> {
        Ok(orders::table
            .filter(orders::columns::market_id.eq(market_id))
            .order(orders::columns::time.asc())
            .load::<QueryableOrder>(&self.conn)?
            .into_iter()
            .map(|raw_order| QueryOrder {
                local_id: raw_order.market_local_id,
                user_id: raw_order.user_id,
                market_id: raw_order.market_id,
                token_name: raw_order.token_name,
                amount_token: raw_order.amount_token,
                amount_coin: raw_order.amount_coin,
                type_: raw_order.type_,
                time: raw_order.time,
            })
            .collect())
    }

    fn query_market_ids_by_status(
        &self,
        status: &[MarketStatus],
    ) -> Result<Vec<Uuid>, failure::Error> {
        Ok(markets::table
            .filter(markets::columns::status.eq(any(status)))
            .select(markets::columns::id)
            .load::<Uuid>(&self.conn)?)
    }

    fn query_market_ids_participated_by_user(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<Uuid>, failure::Error> {
        Ok(orders::table
            .filter(orders::columns::user_id.eq(user_id))
            .select(orders::columns::market_id)
            .distinct()
            .load::<Uuid>(&self.conn)?)
    }

    fn query_market_ids_ready_to_open(&self) -> Result<Vec<Uuid>, failure::Error> {
        Ok(markets::table
            .filter(markets::columns::status.eq(MarketStatus::Upcoming))
            .filter(markets::columns::open.le(now))
            .select(markets::columns::id)
            .load::<Uuid>(&self.conn)?)
    }

    fn query_market_ids_ready_to_close(&self) -> Result<Vec<Uuid>, failure::Error> {
        Ok(markets::table
            .filter(markets::columns::status.eq(MarketStatus::Open))
            .filter(markets::columns::close.le(now))
            .select(markets::columns::id)
            .load::<Uuid>(&self.conn)?)
    }
}

impl QueryMarket {
    fn from_parts(
        raw_market: QueryableMarket,
        raw_tokens: Vec<QueryableToken>,
        raw_reward_records: Option<Vec<QueryableRewardRecords>>,
    ) -> QueryMarket {
        QueryMarket {
            id: raw_market.id,
            title: raw_market.title,
            organizer_id: raw_market.organizer_id,
            description: raw_market.description,
            lmsr_b: raw_market.lmsr_b,
            open: raw_market.open,
            close: raw_market.close,
            status: raw_market.status,
            resolved_token_name: raw_market.resolved_token_name,
            resolved_at: raw_market.resolved_at,
            tokens: raw_tokens
                .into_iter()
                .map(|token| QueryToken {
                    name: token.name,
                    description: token.description,
                    thumbnail_url: token.thumbnail_url,
                })
                .collect(),
            reward_records: raw_reward_records.map(|records| {
                records
                    .into_iter()
                    .map(|record| QueryRewardRecord {
                        user_id: record.user_id,
                        point: record.point as u32,
                    })
                    .collect()
            }),
        }
    }
}

#[derive(Insertable)]
#[table_name = "markets"]
struct InsertableMarket<'a> {
    id: &'a Uuid,
    title: &'a str,
    organizer_id: &'a Uuid,
    description: &'a str,
    lmsr_b: i32,
    open: &'a DateTime<Utc>,
    close: &'a DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "market_tokens"]
struct InsertableToken<'a> {
    name: &'a str,
    description: &'a str,
    thumbnail_url: &'a str,
    market_id: &'a Uuid,
    idx: i32,
}

#[derive(Insertable)]
#[table_name = "orders"]
struct InsertableOrder<'a> {
    market_local_id: i32,
    user_id: Uuid,
    token_name: Option<&'a str>,
    amount_token: i32,
    amount_coin: i32,
    type_: OrderType,
    market_id: Uuid,
    time: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "market_reward_records"]
struct InsertableRewardRecord {
    market_id: Uuid,
    user_id: Uuid,
    point: i32,
}

#[derive(Queryable)]
struct QueryableMarket {
    id: Uuid,
    title: String,
    organizer_id: Uuid,
    description: String,
    lmsr_b: i32,
    open: DateTime<Utc>,
    close: DateTime<Utc>,
    status: MarketStatus,
    resolved_token_name: Option<String>,
    resolved_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Queryable)]
struct QueryableToken {
    _unused_id: i32,
    name: String,
    description: String,
    thumbnail_url: String,
    market_id: Uuid,
    idx: i32,
}

#[derive(Clone, Queryable)]
struct QueryableRewardRecords {
    _unused_id: i32,
    market_id: Uuid,
    user_id: Uuid,
    point: i32,
}

#[derive(Clone, Queryable)]
struct QueryableOrder {
    _unused_id: i32,
    market_local_id: i32,
    user_id: Uuid,
    token_name: Option<String>,
    amount_token: i32,
    amount_coin: i32,
    type_: OrderType,
    time: DateTime<Utc>,
    market_id: Uuid,
}
