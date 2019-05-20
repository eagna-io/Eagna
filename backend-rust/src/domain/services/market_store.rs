use crate::domain::{
    models::{lmsr, market::*, user::UserId},
    services::Store,
};
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum UpdateMarketLastOrderErrorKind<E> {
    Conflict,
    NotOpen,
    Error(E),
}

#[derive(Debug)]
pub enum UpdateMarketStatusErrorKind<E> {
    /// 指定のMarketが存在しない、もしくは既にUpdate処理がなされている場合
    MarketNotFound,
    Error(E),
}

pub trait MarketStore: Store {
    // *************   Required methods ***********

    fn insert_market(&mut self, market: NewMarket) -> Result<MarketId, Self::Error>;

    fn query_market(&mut self, market_id: &MarketId) -> Result<Option<Market>, Self::Error>;

    /// 指定されたUserに紐づくMarketのIDのリストを返す。
    ///
    /// ## NOTE
    /// この関数を直接呼び出すことは基本的にない。
    /// 代わりにquery_markets_related_to_userメソッドを呼び出す。
    fn query_market_ids_related_to_user(
        &mut self,
        user_id: &UserId,
    ) -> Result<Vec<MarketId>, Self::Error>;

    /// open_timeがすでに過ぎているPreparingMarketのIDのリストを返す
    ///
    /// ## NOTE
    /// この関数を直接呼び出すことは基本的にない。
    /// 代わりにquery_markets_ready_to_openメソッドを呼び出す。
    fn query_market_ids_ready_to_open(&mut self) -> Result<Vec<MarketId>, Self::Error>;

    /// close_timeがすでに過ぎているOpenMarketのリストを返す
    ///
    /// ## NOTE
    /// この関数を直接呼び出すことは基本的にない。
    /// 代わりにquery_markets_ready_to_closeメソッドを呼び出す。
    fn query_market_ids_ready_to_close(&mut self) -> Result<Vec<MarketId>, Self::Error>;

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
        &mut self,
        market: &OpenMarket,
    ) -> Result<(), UpdateMarketLastOrderErrorKind<Self::Error>>;

    /// 渡されたOpenMarketのopen処理をstoreに記録する。
    fn update_market_status_to_open(
        &mut self,
        market: &OpenMarket,
    ) -> Result<(), UpdateMarketStatusErrorKind<Self::Error>>;

    /// 渡されたClosedMarketのclose処理をstoreに記録する。
    fn update_market_status_to_closed(
        &mut self,
        market: &ClosedMarket,
    ) -> Result<(), UpdateMarketStatusErrorKind<Self::Error>>;

    // ************* Provided methods ***********

    /// 指定されたUserに紐づくMarketのリストを返す。
    fn query_markets_related_to_user(
        &mut self,
        user_id: &UserId,
    ) -> Result<Vec<Market>, Self::Error> {
        let market_ids = self.query_market_ids_related_to_user(user_id)?;
        let mut vec = Vec::with_capacity(market_ids.len());
        for market_id in market_ids {
            vec.push(self.query_market(&market_id)?.unwrap());
        }
        Ok(vec)
    }

    /// open_timeがすでに過ぎているPreparingMarketのリストを返す
    fn query_markets_ready_to_open(&mut self) -> Result<Vec<PreparingMarket>, Self::Error> {
        let market_ids = self.query_market_ids_ready_to_open()?;
        let mut vec = Vec::with_capacity(market_ids.len());
        for market_id in market_ids {
            match self.query_market(&market_id)?.unwrap() {
                Market::Preparing(m) => vec.push(m),
                _ => panic!("MarketStore::query_market_ids_ready_to_open returns invalid id"),
            }
        }
        Ok(vec)
    }

    /// close_timeがすでに過ぎているOpenMarketのリストを返す
    fn query_markets_ready_to_close(&mut self) -> Result<Vec<OpenMarket>, Self::Error> {
        let market_ids = self.query_market_ids_ready_to_close()?;
        let mut vec = Vec::with_capacity(market_ids.len());
        for market_id in market_ids {
            match self.query_market(&market_id)?.unwrap() {
                Market::Open(m) => vec.push(m),
                _ => panic!("MarketStore::query_market_ids_ready_to_close returns invalid id"),
            }
        }
        Ok(vec)
    }
}

pub struct NewMarket {
    pub title: MarketTitle,
    pub organizer: MarketOrganizer,
    pub short_desc: MarketShortDesc,
    pub description: MarketDesc,
    pub lmsr_b: lmsr::B,
    pub open_time: DateTime<Utc>,
    pub close_time: DateTime<Utc>,
    pub tokens: Vec<NewToken>,
}

pub struct NewToken {
    pub name: TokenName,
    pub description: TokenDesc,
}
