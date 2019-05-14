use crate::domain::models::{
    market::{Market, MarketId, OpenMarket, PreparingMarket},
    user::UserId,
};

#[derive(Debug)]
pub enum UpdateMarketLastOrderResult<E> {
    Success,
    Conflict,
    NotOpen,
    Error(E),
}

#[derive(Debug)]
pub enum UpdateMarketStatusToOpenResult<E> {
    Success,
    NotPrepared,
    Error(E),
}

pub trait MarketStore {
    type Error: std::fmt::Debug;

    fn query_market(&self, market_id: &MarketId) -> Result<Option<Market>, Self::Error>;

    /// 指定されたUserに紐づくMarketのリストを返す。
    /// 紐づくマーケットとは、以下を指す
    /// 1. InitialSupplyを受け取ったMarket
    /// 2. Preparing状態のMarket
    fn query_markets_by_user_id(&self, user_id: &UserId) -> Result<Vec<Market>, Self::Error>;

    /// open_timeがすでに過ぎているPreparingMarketのリストを返す
    fn query_markets_ready_to_open(&self) -> Result<Vec<PreparingMarket>, Self::Error>;

    /// 渡されたOpenMarketのMarketOrdersを更新する。
    /// MarketOrdersはMarketに紐づく「状態」であるので、insertではなく、updateとなる。
    /// もし「状態」がコンフリクトしていたら、このメソッドは失敗する。
    /// また、MarketがStore内でOpen状態でない場合もこのメソッドは失敗する。
    ///
    /// ## NOTE
    /// このメソッドは、updateする差分を計算するときに、最後のOrderのみをチェックする。
    /// つまり、以下のような挙動をする。
    /// 1. 最後のOrderがStoreに存在し、それが等しい場合
    ///   - 何もupdateしない
    /// 2. 最後のOrderがStoreに存在し、それが異なる場合
    ///   - コンフリクトエラー
    /// 3. 最後のOrderがStoreに存在しない場合
    ///   - 最後のOrderを新たに記録する
    ///   - その前のOrderも存在するかどうかはチェックしない
    ///
    /// よって呼び出し元は、Orderを一つ更新するたびにこのメソッドを呼び出す必要がある。
    fn update_market_last_order(
        &self,
        market: &OpenMarket,
    ) -> UpdateMarketLastOrderResult<Self::Error>;

    /// 渡されたOpenMarketのopen処理をstoreに記録する。
    ///
    /// ## Panics
    /// marketに、InitialSupplyOrder以外が格納されていたとき
    fn update_market_status_to_open(
        &self,
        market: &OpenMarket,
    ) -> UpdateMarketStatusToOpenResult<Self::Error>;
}
