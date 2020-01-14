use super::{
    models::{
        ClosedMarket, Market, MarketAttrs, MarketId, MarketStatus, MarketToken, OpenMarket,
        ResolvedMarket, UpcomingMarket,
    },
    num::{AmountCoin, AmountToken},
    order::{MarketOrders, Order, OrderId},
    services::manager::{NewClosedMarket, NewMarket, NewOpenMarket, OpenMarketOrderAdded},
};
use crate::domain::lmsr;
use crate::domain::user::models::UserId;
use crate::infra::postgres::{
    market::{
        NewMarket as InfraNewMarket, NewToken as InfraNewToken, QueryMarket as InfraQueryMarket,
        QueryToken as InfraQueryToken,
    },
    order::{NewOrder as InfraNewOrder, QueryOrder as InfraQueryOrder},
    types::MarketStatus as InfraMarketStatus,
    PostgresInfra,
};
use crate::primitive::{NonEmptyString, NonEmptyVec};

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
    pub fn lock_market(&self, market_id: &MarketId) -> anyhow::Result<()> {
        self.postgres.lock_market(market_id.as_uuid())
    }

    /*
     * Save
     */
    pub fn save_market<M>(&self, market: &M) -> anyhow::Result<()>
    where
        M: InsertableMarket,
    {
        market.insert(self.postgres)
    }

    /*
     * Update
     */
    pub fn update_market<M>(&self, market: &M) -> anyhow::Result<()>
    where
        M: UpdatableMarket,
    {
        market.update(self.postgres)
    }

    /*
     * Query
     */
    pub fn query_market(&self, market_id: &MarketId) -> anyhow::Result<Option<QueryMarket>> {
        match self.postgres.query_market_by_id(market_id.as_uuid())? {
            Some((market, tokens)) => {
                let orders = self
                    .postgres
                    .query_orders_by_market_id(market_id.as_uuid())?;
                Ok(Some(QueryMarket::from_infra(market, tokens, orders)))
            }
            None => Ok(None),
        }
    }

    pub fn query_market_ids_with_status(
        &self,
        status_arr: &[MarketStatus],
    ) -> anyhow::Result<Vec<MarketId>> {
        fn map_status(status: &MarketStatus) -> InfraMarketStatus {
            match status {
                MarketStatus::Upcoming => InfraMarketStatus::Upcoming,
                MarketStatus::Open => InfraMarketStatus::Open,
                MarketStatus::Closed => InfraMarketStatus::Closed,
                MarketStatus::Resolved => InfraMarketStatus::Resolved,
            }
        }
        let infra_status_arr = status_arr.iter().map(map_status).collect::<Vec<_>>();
        Ok(self
            .postgres
            .query_market_ids_by_status(infra_status_arr.as_ref())?
            .into_iter()
            .map(MarketId::from)
            .collect())
    }

    pub fn query_market_ids_ready_to_open(&self) -> anyhow::Result<Vec<MarketId>> {
        Ok(self
            .postgres
            .query_market_ids_ready_to_open()?
            .into_iter()
            .map(MarketId::from)
            .collect())
    }

    pub fn query_market_ids_ready_to_close(&self) -> anyhow::Result<Vec<MarketId>> {
        Ok(self
            .postgres
            .query_market_ids_ready_to_close()?
            .into_iter()
            .map(MarketId::from)
            .collect())
    }
}

/*
 * ===============
 * QueriedMarket
 * ===============
 */
pub struct QueryMarket {
    id: MarketId,
    attrs: MarketAttrs,
    status: MarketStatus,
    orders: MarketOrders,
    resolved_token_name: Option<NonEmptyString>,
}

impl QueryMarket {
    fn from_infra(
        market: InfraQueryMarket,
        tokens: Vec<InfraQueryToken>,
        orders: Vec<InfraQueryOrder>,
    ) -> Self {
        let id = MarketId::from(market.id);

        let market_tokens: Vec<_> = tokens
            .into_iter()
            .map(|token| {
                MarketToken::from((
                    NonEmptyString::from_str(token.name).unwrap(),
                    token.description,
                    token.thumbnail_url,
                ))
            })
            .collect();

        let attrs = MarketAttrs::from((
            NonEmptyString::from_str(market.title).unwrap(),
            market.description,
            lmsr::B::from(market.lmsr_b as u32),
            market.open,
            market.close,
            NonEmptyVec::from_vec(market_tokens).unwrap(),
        ));

        let resolved_token_name = market
            .resolved_token_name
            .map(|s| NonEmptyString::from_str(s).unwrap());

        let status = match market.status {
            InfraMarketStatus::Upcoming => MarketStatus::Upcoming,
            InfraMarketStatus::Open => MarketStatus::Open,
            InfraMarketStatus::Closed => MarketStatus::Closed,
            InfraMarketStatus::Resolved => MarketStatus::Resolved,
        };

        let market_orders = orders
            .into_iter()
            .map(|order| {
                Order::from((
                    OrderId::from(order.id),
                    UserId::from(order.user_id),
                    NonEmptyString::from_str(order.token_name).unwrap(),
                    AmountToken::from(order.amount_token),
                    AmountCoin::from(order.amount_coin),
                    order.time,
                ))
            })
            .collect::<Vec<_>>();

        QueryMarket {
            id,
            attrs,
            status,
            orders: MarketOrders::from(market_orders),
            resolved_token_name,
        }
    }
}

macro_rules! impl_market {
    ($t:ty) => {
        impl Market for $t {
            fn id(&self) -> MarketId {
                self.id
            }
            fn attrs(&self) -> &MarketAttrs {
                &self.attrs
            }
            fn status(&self) -> MarketStatus {
                self.status
            }
            fn orders(&self) -> &MarketOrders {
                &self.orders
            }
        }
    };
}

impl_market!(QueryMarket);

impl QueryMarket {
    pub fn into_upcoming_market(self) -> Option<impl UpcomingMarket>
    where
        Self: Sized,
    {
        if self.status() == MarketStatus::Upcoming {
            Some(Market_UpcomingMarket {
                id: self.id,
                attrs: self.attrs,
                status: self.status,
                orders: self.orders,
            })
        } else {
            None
        }
    }

    pub fn into_open_market(self) -> Option<impl OpenMarket>
    where
        Self: Sized,
    {
        if self.status() == MarketStatus::Upcoming {
            Some(Market_OpenMarket {
                id: self.id,
                attrs: self.attrs,
                status: self.status,
                orders: self.orders,
            })
        } else {
            None
        }
    }

    pub fn into_closed_market(self) -> Option<impl ClosedMarket>
    where
        Self: Sized,
    {
        if self.status() == MarketStatus::Upcoming {
            Some(Market_ClosedMarket {
                id: self.id,
                attrs: self.attrs,
                status: self.status,
                orders: self.orders,
            })
        } else {
            None
        }
    }

    pub fn into_resolved_market(self) -> Option<impl ResolvedMarket>
    where
        Self: Sized,
    {
        if self.status() == MarketStatus::Upcoming {
            Some(Market_ResolvedMarket {
                id: self.id,
                attrs: self.attrs,
                status: self.status,
                orders: self.orders,
                resolved_token_name: self.resolved_token_name.unwrap(),
            })
        } else {
            None
        }
    }
}

#[allow(non_camel_case_types)]
struct Market_UpcomingMarket {
    id: MarketId,
    attrs: MarketAttrs,
    status: MarketStatus,
    orders: MarketOrders,
}
impl_market!(Market_UpcomingMarket);
impl UpcomingMarket for Market_UpcomingMarket {}

#[allow(non_camel_case_types)]
struct Market_OpenMarket {
    id: MarketId,
    attrs: MarketAttrs,
    status: MarketStatus,
    orders: MarketOrders,
}
impl_market!(Market_OpenMarket);
impl OpenMarket for Market_OpenMarket {}

#[allow(non_camel_case_types)]
struct Market_ClosedMarket {
    id: MarketId,
    attrs: MarketAttrs,
    status: MarketStatus,
    orders: MarketOrders,
}
impl_market!(Market_ClosedMarket);
impl ClosedMarket for Market_ClosedMarket {}

#[allow(non_camel_case_types)]
struct Market_ResolvedMarket {
    id: MarketId,
    attrs: MarketAttrs,
    status: MarketStatus,
    orders: MarketOrders,
    resolved_token_name: NonEmptyString,
}
impl_market!(Market_ResolvedMarket);
impl ResolvedMarket for Market_ResolvedMarket {
    fn resolved_token_name(&self) -> &NonEmptyString {
        &self.resolved_token_name
    }
}

/*
 * ==================
 * InsertableMarket
 * ==================
 */
pub trait InsertableMarket {
    fn insert(&self, pg: &dyn PostgresInfra) -> anyhow::Result<()>;
}

impl InsertableMarket for NewMarket {
    fn insert(&self, pg: &dyn PostgresInfra) -> anyhow::Result<()> {
        let new_market = InfraNewMarket {
            id: *self.id().as_uuid(),
            title: self.attrs().title().as_str(),
            description: self.attrs().description().as_str(),
            lmsr_b: self.attrs().lmsr_b().as_u32() as i32,
            open: *self.attrs().open(),
            close: *self.attrs().close(),
        };

        let new_tokens = self
            .attrs()
            .tokens()
            .iter()
            .enumerate()
            .map(|(idx, token)| InfraNewToken {
                name: token.name().as_str(),
                description: token.description().as_str(),
                thumbnail_url: token.thumbnail_url().as_str(),
                market_id: *self.id().as_uuid(),
                idx: idx as i32,
            })
            .collect::<Vec<_>>();

        pg.insert_market(&new_market, &new_tokens)
    }
}

/*
 * ===================
 * UpdatableMarket
 * ===================
 */
pub trait UpdatableMarket {
    fn update(&self, pg: &dyn PostgresInfra) -> anyhow::Result<()>;
}

impl<M: Market> UpdatableMarket for NewOpenMarket<M> {
    fn update(&self, pg: &dyn PostgresInfra) -> anyhow::Result<()> {
        pg.update_market_status(self.id().as_uuid(), &InfraMarketStatus::Open)
    }
}

impl<M: Market> UpdatableMarket for OpenMarketOrderAdded<M> {
    fn update(&self, pg: &dyn PostgresInfra) -> anyhow::Result<()> {
        let order = self.added_order();
        let new_order = InfraNewOrder {
            id: order.id().as_uuid(),
            user_id: *order.user_id().as_uuid(),
            token_name: order.token_name().as_str(),
            amount_token: order.amount_token().as_i32(),
            amount_coin: order.amount_coin().as_i32(),
            time: *order.time(),
            market_id: *self.id().as_uuid(),
        };
        pg.insert_order(&new_order)
    }
}

impl<M: Market> UpdatableMarket for NewClosedMarket<M> {
    fn update(&self, pg: &dyn PostgresInfra) -> anyhow::Result<()> {
        pg.update_market_status(self.id().as_uuid(), &InfraMarketStatus::Closed)
    }
}
