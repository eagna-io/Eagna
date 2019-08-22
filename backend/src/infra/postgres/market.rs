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

    fn update_market_status_and_resolved_token_name(
        &self,
        market_id: &Uuid,
        market_status: &MarketStatus,
        resolved_token_name: &str,
    ) -> Result<(), failure::Error>;

    fn insert_orders<'a>(
        &self,
        market_id: &'a Uuid,
        orders: &'a mut dyn Iterator<Item = NewOrder<'a>>,
    ) -> Result<(), failure::Error>;

    fn query_markets_by_ids(&self, ids: &[Uuid]) -> Result<Vec<QueryMarket>, failure::Error>;

    /// 時系列順にソートされた `QueryOrder` を返す。
    /// 古いものが最初に、新しいものが最後に来る
    fn query_orders_by_market_ids(
        &self,
        market_ids: &[Uuid],
    ) -> Result<Vec<QueryOrder>, failure::Error>;

    fn query_market_ids_by_status(
        &self,
        status: &[MarketStatus],
    ) -> Result<Vec<Uuid>, failure::Error>;

    fn query_market_ids_participated_by_user(
        &self,
        user_id: &str,
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
    pub tokens: &'a mut dyn Iterator<Item = NewToken<'a>>,
    pub prizes: &'a mut dyn Iterator<Item = NewPrize<'a>>,
    // status は常にUpcoming
}

pub struct NewToken<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub sumbnail_url: &'a str,
}

pub struct NewPrize<'a> {
    pub local_id: i32,
    pub name: &'a str,
    pub sumbnail_url: &'a str,
    pub target: &'a str,
}

pub struct NewOrder<'a> {
    pub local_id: i32,
    pub user_id: &'a str,
    pub token_name: Option<&'a str>,
    pub amount_token: i32,
    pub amount_coin: i32,
    pub type_: OrderType,
    pub time: DateTime<Utc>,
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
    pub resolved_token_name: Option<String>,
    pub tokens: Vec<QueryToken>,
    pub prizes: Vec<QueryPrize>,
}

#[derive(Debug, Clone)]
pub struct QueryToken {
    pub name: String,
    pub description: String,
    pub sumbnail_url: String,
}

#[derive(Debug, Clone)]
pub struct QueryPrize {
    pub local_id: i32,
    pub name: String,
    pub sumbnail_url: String,
    pub target: String,
}

#[derive(Debug, Clone)]
pub struct QueryOrder {
    pub local_id: i32,
    pub user_id: String,
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
                sumbnail_url: token.sumbnail_url,
                market_id: &market_id,
            })
            .collect();
        diesel::insert_into(market_tokens::table)
            .values(insert_tokens)
            .execute(&self.conn)?;

        // Insert prizes
        let insert_prizes: Vec<_> = market
            .prizes
            .map(|prize| InsertablePrize {
                market_local_id: prize.local_id,
                name: prize.name,
                sumbnail_url: prize.sumbnail_url,
                target: prize.target,
                market_id: &market_id,
            })
            .collect();
        diesel::insert_into(market_prizes::table)
            .values(insert_prizes)
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

    fn update_market_status_and_resolved_token_name(
        &self,
        market_id: &Uuid,
        market_status: &MarketStatus,
        resolved_token_name: &str,
    ) -> Result<(), failure::Error> {
        diesel::update(markets::table.filter(markets::id.eq(market_id)))
            .set((
                markets::status.eq(market_status),
                markets::resolved_token_name.eq(resolved_token_name),
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
                market_id: market_id,
            })
            .collect();
        diesel::insert_into(orders::table)
            .values(&insert_orders)
            .execute(&self.conn)?;
        Ok(())
    }

    fn query_markets_by_ids(&self, ids: &[Uuid]) -> Result<Vec<QueryMarket>, failure::Error> {
        let raw_markets = markets::table
            .filter(markets::columns::id.eq(any(ids)))
            .order(markets::columns::id.desc())
            .load::<QueryableMarket>(&self.conn)?;
        let raw_market_tokens = market_tokens::table
            .filter(market_tokens::columns::market_id.eq(any(ids)))
            .order(market_tokens::columns::market_id.desc())
            .load::<QueryableToken>(&self.conn)?;
        let raw_market_prizes = market_prizes::table
            .filter(market_prizes::columns::market_id.eq(any(ids)))
            .order(market_prizes::columns::market_id.desc())
            .load::<QueryablePrize>(&self.conn)?;

        let mut constructed_markets = Vec::with_capacity(raw_markets.len());

        for raw_market in raw_markets {
            let market_id = raw_market.id;
            let corresponding_tokens = raw_market_tokens
                .iter()
                .filter(|token| token.market_id == market_id)
                .cloned();
            let corresponding_prizes = raw_market_prizes
                .iter()
                .filter(|prize| prize.market_id == market_id)
                .cloned();

            let constructed_market = construct_query_market_from_parts(
                raw_market,
                corresponding_tokens,
                corresponding_prizes,
            );
            constructed_markets.push(constructed_market);
        }

        Ok(constructed_markets)
    }

    /// 時系列順にソートされた `QueryOrder` を返す。
    /// 古いものが最初に、新しいものが最後に来る
    fn query_orders_by_market_ids(
        &self,
        market_ids: &[Uuid],
    ) -> Result<Vec<QueryOrder>, failure::Error> {
        Ok(orders::table
            .filter(orders::columns::market_id.eq(any(market_ids)))
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
        user_id: &str,
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

fn construct_query_market_from_parts<IT, IP>(
    raw_market: QueryableMarket,
    raw_tokens: IT,
    raw_prizes: IP,
) -> QueryMarket
where
    IT: Iterator<Item = QueryableToken>,
    IP: Iterator<Item = QueryablePrize>,
{
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
        tokens: raw_tokens
            .map(|token| QueryToken {
                name: token.name,
                description: token.description,
                sumbnail_url: token.sumbnail_url,
            })
            .collect(),
        prizes: raw_prizes
            .map(|prize| QueryPrize {
                local_id: prize.market_local_id,
                name: prize.name,
                sumbnail_url: prize.sumbnail_url,
                target: prize.target,
            })
            .collect(),
    }
}

use super::schema::{market_prizes, market_tokens, markets, orders};

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
    sumbnail_url: &'a str,
    market_id: &'a Uuid,
}

#[derive(Insertable)]
#[table_name = "market_prizes"]
struct InsertablePrize<'a> {
    market_local_id: i32,
    name: &'a str,
    sumbnail_url: &'a str,
    target: &'a str,
    market_id: &'a Uuid,
}

#[derive(Insertable)]
#[table_name = "orders"]
struct InsertableOrder<'a> {
    market_local_id: i32,
    user_id: &'a str,
    token_name: Option<&'a str>,
    amount_token: i32,
    amount_coin: i32,
    type_: OrderType,
    market_id: &'a Uuid,
    time: DateTime<Utc>,
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
}

#[derive(Clone, Queryable)]
struct QueryableToken {
    _unused_id: i32,
    name: String,
    description: String,
    sumbnail_url: String,
    market_id: Uuid,
}

#[derive(Clone, Queryable)]
struct QueryablePrize {
    _unused_id: i32,
    market_local_id: i32,
    name: String,
    sumbnail_url: String,
    target: String,
    market_id: Uuid,
}

#[derive(Clone, Queryable)]
struct QueryableOrder {
    _unused_id: i32,
    market_local_id: i32,
    user_id: String,
    token_name: Option<String>,
    amount_token: i32,
    amount_coin: i32,
    type_: OrderType,
    time: DateTime<Utc>,
    market_id: Uuid,
}
