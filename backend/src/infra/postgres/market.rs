use super::{
    schema::{market_tokens, markets},
    types::MarketStatus,
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
    fn lock_market(&self, market_id: &Uuid) -> anyhow::Result<()>;

    fn insert_market<'a>(
        &self,
        market: &NewMarket<'a>,
        tokens: &Vec<NewToken<'a>>,
    ) -> anyhow::Result<()>;

    fn update_market_status(
        &self,
        market_id: &Uuid,
        market_status: &MarketStatus,
    ) -> anyhow::Result<()>;

    /// - market_statusをResolvedに変更
    /// - resolved_token_name を設定
    /// - resolved_at を設定
    fn resolve_market(&self, market_id: &Uuid, resolved_token_name: &str) -> anyhow::Result<()>;

    fn query_market_by_id(
        &self,
        id: &Uuid,
    ) -> anyhow::Result<Option<(QueryMarket, Vec<QueryToken>)>>;

    fn query_market_ids_by_status(&self, status: &[MarketStatus]) -> anyhow::Result<Vec<Uuid>>;

    fn query_market_ids_ready_to_open(&self) -> anyhow::Result<Vec<Uuid>>;

    fn query_market_ids_ready_to_close(&self) -> anyhow::Result<Vec<Uuid>>;
}

#[derive(Debug, Insertable)]
#[table_name = "markets"]
pub struct NewMarket<'a> {
    pub id: Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub lmsr_b: i32,
    pub open: DateTime<Utc>,
    pub close: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[table_name = "market_tokens"]
pub struct NewToken<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub thumbnail_url: &'a str,
    pub market_id: Uuid,
    pub idx: i32,
}

#[derive(Debug, Clone, Queryable)]
pub struct QueryMarket {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub lmsr_b: i32,
    pub open: DateTime<Utc>,
    pub close: DateTime<Utc>,
    pub status: MarketStatus,
    pub resolved_token_name: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Queryable)]
pub struct QueryToken {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub thumbnail_url: String,
    pub market_id: Uuid,
    pub idx: i32,
}
/*
 * ==================
 * Implementation
 * ==================
 */

impl PostgresMarketInfra for Postgres {
    fn lock_market(&self, market_id: &Uuid) -> anyhow::Result<()> {
        markets::table
            .select(markets::columns::id)
            .filter(markets::columns::id.eq(market_id))
            .for_update()
            .first::<Uuid>(&self.conn)?;
        Ok(())
    }
    fn insert_market<'a>(
        &self,
        market: &NewMarket<'a>,
        tokens: &Vec<NewToken<'a>>,
    ) -> anyhow::Result<()> {
        // Insert market
        diesel::insert_into(markets::table)
            .values(market)
            .execute(&self.conn)?;

        // Insert tokens
        diesel::insert_into(market_tokens::table)
            .values(tokens)
            .execute(&self.conn)?;

        Ok(())
    }

    fn update_market_status(&self, market_id: &Uuid, status: &MarketStatus) -> anyhow::Result<()> {
        diesel::update(markets::table.filter(markets::id.eq(market_id)))
            .set(markets::status.eq(status))
            .execute(&self.conn)?;
        Ok(())
    }

    fn resolve_market(&self, market_id: &Uuid, resolved_token_name: &str) -> anyhow::Result<()> {
        diesel::update(markets::table.filter(markets::id.eq(market_id)))
            .set((
                markets::status.eq(MarketStatus::Resolved),
                markets::resolved_token_name.eq(resolved_token_name),
                markets::resolved_at.eq(Utc::now()),
            ))
            .execute(&self.conn)?;
        Ok(())
    }

    // # NOTE
    // tokens は market_tokens テーブルの idx カラムの順に取り出される。
    fn query_market_by_id(
        &self,
        id: &Uuid,
    ) -> anyhow::Result<Option<(QueryMarket, Vec<QueryToken>)>> {
        // QueryableMarket を取得
        let market = match markets::table
            .find(id)
            .first::<QueryMarket>(&self.conn)
            .optional()?
        {
            None => return Ok(None),
            Some(m) => m,
        };

        // idxカラムの順にQueryTokenを取得
        let tokens = market_tokens::table
            .filter(market_tokens::columns::market_id.eq(id))
            .order(market_tokens::columns::idx.asc())
            .load::<QueryToken>(&self.conn)?;

        Ok(Some((market, tokens)))
    }

    fn query_market_ids_by_status(&self, status: &[MarketStatus]) -> anyhow::Result<Vec<Uuid>> {
        Ok(markets::table
            .filter(markets::columns::status.eq(any(status)))
            .select(markets::columns::id)
            .load::<Uuid>(&self.conn)?)
    }

    fn query_market_ids_ready_to_open(&self) -> anyhow::Result<Vec<Uuid>> {
        Ok(markets::table
            .filter(markets::columns::status.eq(MarketStatus::Upcoming))
            .filter(markets::columns::open.le(now))
            .select(markets::columns::id)
            .load::<Uuid>(&self.conn)?)
    }

    fn query_market_ids_ready_to_close(&self) -> anyhow::Result<Vec<Uuid>> {
        Ok(markets::table
            .filter(markets::columns::status.eq(MarketStatus::Open))
            .filter(markets::columns::close.le(now))
            .select(markets::columns::id)
            .load::<Uuid>(&self.conn)?)
    }
}
