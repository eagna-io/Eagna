use super::{schema::orders, Postgres};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

/*
 * =================
 * Interface
 * =================
 */

pub trait PostgresOrderInfra {
    fn insert_order<'a>(&self, order: &NewOrder<'a>) -> anyhow::Result<()>;

    fn query_orders_by_market_id(&self, market_id: &Uuid) -> anyhow::Result<Vec<QueryOrder>>;
}

#[derive(Insertable)]
#[table_name = "orders"]
pub struct NewOrder<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_name: &'a str,
    pub amount_token: i32,
    pub amount_coin: i32,
    pub time: DateTime<Utc>,
    pub market_id: Uuid,
}

#[derive(Queryable)]
pub struct QueryOrder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_name: String,
    pub amount_token: i32,
    pub amount_coin: i32,
    pub time: DateTime<Utc>,
    pub market_id: Uuid,
}

impl PostgresOrderInfra for Postgres {
    fn insert_order<'a>(&self, order: &NewOrder<'a>) -> anyhow::Result<()> {
        diesel::insert_into(orders::table)
            .values(order)
            .execute(&self.conn)?;
        Ok(())
    }

    /// older first, newer last
    fn query_orders_by_market_id(&self, market_id: &Uuid) -> anyhow::Result<Vec<QueryOrder>> {
        orders::table
            .filter(orders::market_id.eq(market_id))
            .order(orders::columns::time.asc())
            .load::<QueryOrder>(&self.conn)
            .map_err(anyhow::Error::from)
    }
}
