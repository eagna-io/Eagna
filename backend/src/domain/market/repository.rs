mod query;
mod save;
use super::*;
use crate::infra::postgres::PostgresInfra;

#[derive(From)]
/// `MarketRepository` の生成には `MarketRepository::from` を使う.
/// `new` メソッドを提供しないのは、リポジトリのモデル的なライフサイクルを明確にするため。
/// つまり、リポジトリはモデル的にはプログラムの実行前から実行後までずっと存在する。
/// よってプログラム上では新規作成するのではなく、再構築するという表現の方が正しい。
pub struct MarketRepository<'a> {
    postgres: &'a dyn PostgresInfra,
}

impl<'a> MarketRepository<'a> {
    /*
     * Lock
     */
    pub fn lock_market(&self, market_id: &MarketId) -> Result<(), failure::Error> {
        self.postgres.lock_market(market_id.as_uuid())
    }

    /*
     * Save
     */
    pub fn save_market(&self, market: &Market) -> Result<(), failure::Error> {
        save::save_market(self.postgres, market)
    }

    /*
     * Query
     */
    pub fn query_market(&self, market_id: &MarketId) -> Result<Option<Market>, failure::Error> {
        query::query_market(self.postgres, market_id)
    }

    pub fn query_markets(&self, market_ids: &[MarketId]) -> Result<Vec<Market>, failure::Error> {
        query::query_markets(self.postgres, market_ids)
    }

    pub fn query_market_ids_with_status(
        &self,
        statuses: &[MarketStatus],
    ) -> Result<Vec<MarketId>, failure::Error> {
        query::query_market_ids_with_status(self.postgres, statuses)
    }

    pub fn query_market_ids_participated_by_user(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<MarketId>, failure::Error> {
        query::query_market_ids_participated_by_user(self.postgres, user_id)
    }

    pub fn query_market_ids_ready_to_open(&self) -> Result<Vec<MarketId>, failure::Error> {
        query::query_market_ids_ready_to_open(self.postgres)
    }

    pub fn query_market_ids_ready_to_close(&self) -> Result<Vec<MarketId>, failure::Error> {
        query::query_market_ids_ready_to_close(self.postgres)
    }
}
