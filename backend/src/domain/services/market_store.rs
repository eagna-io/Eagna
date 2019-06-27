use crate::domain::{
    models::{lmsr, market::*, user::UserId},
    services::Store,
};
use chrono::{DateTime, Utc};

pub trait MarketStore: Store + Sized {
    /// 指定されたMarketに対するロックを獲得する。
    /// ロックはトランザクションが終わるまで有効である。
    /// ロックを複数回獲得しても影響はない
    fn lock_market_inner(&mut self, market_id: &MarketId) -> Result<(), Self::Error>;

    /// Marketに対するロックを獲得する
    /// ロックはトランザクションが終わるまで有効である。
    /// ロックを複数回獲得しても影響はない
    fn lock_market(
        &mut self,
        market_id: &MarketId,
    ) -> Result<LockedMarketStore<Self>, Self::Error> {
        self.lock_market_inner(market_id)?;
        Ok(LockedMarketStore {
            inner: self,
            market_id: *market_id,
        })
    }

    fn insert_market(&mut self, market: NewMarket) -> Result<MarketId, Self::Error>;

    fn query_market(&mut self, market_id: &MarketId) -> Result<Option<Market>, Self::Error>;

    fn query_market_ids_with_status<I>(&mut self, status: I) -> Result<Vec<MarketId>, Self::Error>
    where
        I: Iterator<Item = MarketStatus>;

    /// 指定されたUserに紐づくMarketのIDのリストを返す。
    /// つまり、対象のUserに対してInitialSupplyを配布しているMarket。
    fn query_market_ids_related_to_user(
        &mut self,
        user_id: &UserId,
    ) -> Result<Vec<MarketId>, Self::Error>;

    /// open_timeがすでに過ぎているPreparingMarketのIDのリストを返す
    fn query_market_ids_ready_to_open(&mut self) -> Result<Vec<MarketId>, Self::Error>;

    /// close_timeがすでに過ぎているOpenMarketのIDのリストを返す
    fn query_market_ids_ready_to_close(&mut self) -> Result<Vec<MarketId>, Self::Error>;

    /// 指定されたMarketのstatusを変更する
    ///
    /// ## NOTE
    /// 利用者はこのメソッドを直接使用するべきではない
    fn update_market_status(
        &mut self,
        market_id: &MarketId,
        status: &MarketStatus,
    ) -> Result<(), Self::Error>;

    /// 指定されたMarketのstatusを Settle に変更し、
    /// settle_token_id をセットする
    fn update_market_status_and_settle_token(
        &mut self,
        market_id: &MarketId,
        settle_token_id: &TokenId,
    ) -> Result<(), Self::Error>;

    /// MarketにOrderを追加する。
    ///
    /// ## NOTE
    /// 利用者はこのメソッドを直接使用するべきではない
    fn insert_market_orders<'a, I>(
        &mut self,
        market_id: &MarketId,
        orders: I,
    ) -> Result<(), Self::Error>
    where
        I: Iterator<Item = (OrderId, &'a Order)>;
}

pub struct LockedMarketStore<'a, S> {
    inner: &'a mut S,
    market_id: MarketId,
}

impl<'a, S> LockedMarketStore<'a, S>
where
    S: MarketStore,
{
    /// 渡されたOpenMarketのMarketOrdersを更新する。
    /// MarketOrdersはMarketに紐づく「状態」であるので、insertではなく、updateとなる。
    /// もし「状態」がコンフリクトしていたら、このメソッドは失敗する。
    ///
    /// ## NOTE
    /// このメソッドは、Marketが現在Openかどうかをチェックしない。
    /// よって呼び出し元は、MarketがOpenであり、ロックされていることを保証する必要がある
    ///
    /// ## NOTE
    /// このメソッドは、updateする差分を計算するときに、最後のOrderのみをチェックする。
    /// つまり、以下のような挙動をする。
    /// 1. 最後のOrderと同じOrderIdがStoreに存在するとき
    ///   - コンフリクトエラー
    /// 3. 最後のOrderがStoreに存在しない場合
    ///   - 最後のOrderを新たに記録する
    ///   - **その前のOrderも存在するかどうかはチェックしない**
    ///
    /// よって呼び出し元は、Orderを一つ更新するたびにこのメソッドを呼び出す必要がある。
    pub fn update_market_last_order(&mut self, market: &OpenMarket) -> Result<(), S::Error> {
        assert_eq!(self.market_id, market.base.id);
        let (serial_num, last_order) = market.last_order().unwrap();
        self.inner
            .insert_market_orders(&self.market_id, std::iter::once((serial_num, last_order)))
    }

    /// 渡されたOpenMarketのopen処理をstoreに記録する。
    ///
    /// ## NOTE
    /// このメソッドは、Marketが現在Preparingかどうかをチェックしない。
    /// よって呼び出し元は、MarketがPreparingであり、
    /// ロックされていることを保証する必要がある。
    pub fn update_market_status_to_open(&mut self, market: &OpenMarket) -> Result<(), S::Error> {
        assert_eq!(self.market_id, market.base.id);
        self.inner
            .insert_market_orders(&self.market_id, market.orders.iter())?;
        self.inner
            .update_market_status(&self.market_id, &MarketStatus::Open)
    }

    /// 渡されたClosedMarketのclose処理をstoreに記録する。
    ///
    /// ## NOTE
    /// このメソッドは、Marketが現在Openかどうかをチェックしない。
    /// よって呼び出し元は、MarketがOpenであり、
    /// ロックされていることを保証する必要がある。
    pub fn update_market_status_to_closed(
        &mut self,
        market: &ClosedMarket,
    ) -> Result<(), S::Error> {
        assert_eq!(self.market_id, market.base.id);
        self.inner
            .update_market_status(&self.market_id, &MarketStatus::Closed)
    }

    /// 渡されたClosedMarketのsettle処理をstoreに記録する。
    ///
    /// ## NOTE
    /// このメソッドは、Marketが現在Closeかどうかをチェックしない。
    /// よって呼び出し元は、MarketがCloseであり、
    /// ロックされていることを保証する必要がある。
    pub fn update_market_status_to_settle(
        &mut self,
        market: &SettledMarket,
    ) -> Result<(), S::Error> {
        assert_eq!(self.market_id, market.base.id);
        let settle_orders = market.orders.iter().filter(|(_id, order)| match order {
            Order::Settle(_) => true,
            _ => false,
        });
        self.inner
            .insert_market_orders(&self.market_id, settle_orders)?;
        self.inner
            .update_market_status_and_settle_token(&self.market_id, &market.settle_token.id)
    }
}

impl<'a, S> std::ops::Deref for LockedMarketStore<'a, S> {
    type Target = S;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, S> std::ops::DerefMut for LockedMarketStore<'a, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
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
