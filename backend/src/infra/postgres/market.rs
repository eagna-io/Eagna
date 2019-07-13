use super::{
    types::{MarketStatus, OrderType},
    Postgres,
};
use chrono::{DateTime, Utc};
use diesel::result::Error as PgError;
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
        orders: &'a dyn Iterator<Item = NewOrder<'a>>,
    ) -> Result<(), failure::Error>;

    fn query_market_by_id(&self, market_id: &Uuid) -> Result<Option<QueryMarket>, failure::Error>;

    fn query_orders_by_market_ids<'a>(
        &'a self,
        market_ids: &'a dyn Iterator<Item = &'a Uuid>,
    ) -> Result<Vec<QueryOrder>, failure::Error>;

    fn query_markets_by_status(
        &self,
        status: &dyn Iterator<Item = MarketStatus>,
    ) -> Result<Vec<QueryMarket>, failure::Error>;

    fn query_markets_by_user_id(&self, user_id: &str) -> Result<Vec<QueryMarket>, failure::Error>;

    fn query_markets_ready_to_open(&self) -> Result<Vec<QueryMarket>, failure::Error>;

    fn query_markets_ready_to_close(&self) -> Result<Vec<QueryMarket>, failure::Error>;
}

pub struct NewMarket<'a> {
    pub id: &'a Uuid,
    pub title: &'a str,
    pub organizer_id: &'a Uuid,
    pub description: &'a str,
    pub lmsr_b: i32,
    pub open: &'a DateTime<Utc>,
    pub close: &'a DateTime<Utc>,
    pub tokens: &'a dyn Iterator<Item = NewToken<'a>>,
    pub prizes: &'a dyn Iterator<Item = NewPrize<'a>>,
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

pub struct QueryToken {
    pub name: String,
    pub description: String,
    pub sumbnail_url: String,
}

pub struct QueryPrize {
    pub local_id: i32,
    pub name: String,
    pub sumbnail_url: String,
    pub target: String,
}

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
            .first(&self.conn)?;
        Ok(())
    }

    fn insert_upcoming_market<'a>(&self, market: NewMarket<'a>) -> Result<(), failure::Error> {
        // Insert market
        let insert_market = InsertableMarket {
            id: market.id,
            title: market.title,
            organizer_id: market.organizer.id,
            description: market.description,
            lmsr_b: market.lmsr_b,
            open: market.open,
            close: market.close,
        };
        diesel::insert_into(markets::table)
            .values(insert_market)
            .execute(&self.conn)?;

        // Insert tokens
        let insert_tokens: Vec<_> = market
            .tokens
            .map(|token| InsertableToken {
                name: token.name,
                description: token.description,
                sumbnail_url: token.sumbnail_url,
                market_id: market.id,
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
                market_id: market.id,
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
        orders: &'a dyn Iterator<Item = NewOrder<'a>>,
    ) -> Result<(), failure::Error> {
        let insert_orders = orders
            .map(|order| InsertableOrder {
                market_local_id: order.local_id,
                user_id: order.user_id,
                token_name: order.token_name,
                amount_token: order.amount_token,
                amount_coin: order.amount_coin,
                type_: order.type_,
                market_id: market_id,
            })
            .collect();
        diesel::insert_into(orders::table)
            .values()
            .execute(&self.conn)?;
    }

    fn query_market_by_id(&self, market_id: &Uuid) -> Result<Option<QueryMarket>, failure::Error> {
        // Query market
        let res = markets::table
            .find(market_id)
            .first::<QueryableMarket>(&self.conn);
        let market = match res {
            Ok(m) => m,
            Err(PgError::NotFound) => return Ok(None),
            Err(e) => return Err(e),
        };

        // Query market tokens
        let tokens = market_tokens::table
            .filter(market_tokens::columns::market_id.eq(market_id))
            .load::<QueryableToken>(&self.conn)?;

        // Query market prizes
        let prizes = market_prizes::table
            .filter(market_prizes::columns::market_id.eq(market_id))
            .load::<QueryablePrize>(&self.conn)?;

        Ok(QueryMarket {
            id: *market_id,
            title: market.title,
            organizer_id: market.organizer_id,
            description: market.description,
            lmsr_b: market.lmsr_b,
            open: market.open,
            close: market.close,
            status: market.status,
            resolved_token_name: market.resolved_token_name,
            tokens: tokens
                .map(|token| QueryToken {
                    name: token.name,
                    description: token.description,
                    sumbnail_url: token.sumbnail_url,
                })
                .collect(),
            prizes: prizes
                .map(|prize| QueryPrize {
                    local_id: prize.market_local_id,
                    name: prize.name,
                    sumbnail_url: prize.sumbnail_url,
                    target: prize.target,
                })
                .collect(),
        })
    }

    fn query_orders_by_market_ids<'a>(
        &self,
        market_id: &'a dyn Iterator<Item = &'a Uuid>,
    ) -> Result<Vec<QueryOrder>, failure::Error> {
        unimplemented!();
    }

    fn query_markets_by_status(
        &self,
        status: &dyn Iterator<Item = MarketStatus>,
    ) -> Result<Vec<QueryMarket>, failure::Error> {
        unimplemented!();
    }

    fn query_markets_by_user_id(&self, user_id: &str) -> Result<Vec<QueryMarket>, failure::Error> {
        unimplemented!();
    }

    fn query_markets_ready_to_open(&self) -> Result<Vec<QueryMarket>, failure::Error> {
        unimplemented!();
    }

    fn query_markets_ready_to_close(&self) -> Result<Vec<QueryMarket>, failure::Error> {
        unimplemented!();
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
    open: DateTime<Utc>,
    close: DateTime<Utc>,
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
#[table_name = "markets"]
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

#[derive(Queryable)]
#[table_name = "market_tokens"]
struct QueryableToken {
    unused_id: i32,
    name: String,
    description: String,
    sumbnail_url: String,
    market_id: Uuid,
}

#[derive(Queryable)]
#[table_name = "market_tokens"]
struct QueryablePrize {
    unused_id: i32,
    market_local_id: i32,
    name: String,
    sumbnail_url: String,
    target: String,
    market_id: Uuid,
}
