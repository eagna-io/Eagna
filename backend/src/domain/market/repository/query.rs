use super::*;
use crate::domain::{organizer::OrganizerId, point::Point};
use crate::infra::postgres::{
    market::*,
    types::{MarketStatus as InfraMarketStatus, OrderType as InfraOrderType},
    PostgresInfra,
};
use crate::primitive::{NonEmptyString, NonEmptyVec};
use arrayvec::ArrayVec;

pub fn query_market(
    postgres: &dyn PostgresInfra,
    market_id: &MarketId,
) -> Result<Option<Market>, failure::Error> {
    let raw_market = match postgres.query_market_by_id(market_id.as_uuid())? {
        None => return Ok(None),
        Some(m) => m,
    };
    let raw_orders = postgres.query_orders_by_market_id(market_id.as_uuid())?;
    Ok(Some(build_market(raw_market, raw_orders)))
}

pub fn query_markets(
    postgres: &dyn PostgresInfra,
    market_ids: &[MarketId],
) -> Result<Vec<Market>, failure::Error> {
    let mut markets = Vec::with_capacity(market_ids.len());
    // TODO
    // Parallelに実行できるようにする
    for market_id in market_ids {
        if let Some(market) = query_market(postgres, market_id)? {
            markets.push(market);
        }
    }
    Ok(markets)
}

pub fn query_market_ids_with_status(
    postgres: &dyn PostgresInfra,
    statuses: &[MarketStatus],
) -> Result<Vec<MarketId>, failure::Error> {
    let mut infra_statuses = ArrayVec::<[InfraMarketStatus; 4]>::new();
    for status in statuses {
        let _ = infra_statuses.try_push(convert_marktet_status_to_infra(*status));
    }

    Ok(postgres
        .query_market_ids_by_status(infra_statuses.as_slice())?
        .into_iter()
        .map(MarketId::from)
        .collect())
}

pub fn query_market_ids_participated_by_user(
    postgres: &dyn PostgresInfra,
    user_id: &UserId,
) -> Result<Vec<MarketId>, failure::Error> {
    Ok(postgres
        .query_market_ids_participated_by_user(user_id.as_uuid())?
        .into_iter()
        .map(MarketId::from)
        .collect())
}

pub fn query_market_ids_ready_to_open(
    postgres: &dyn PostgresInfra,
) -> Result<Vec<MarketId>, failure::Error> {
    Ok(postgres
        .query_market_ids_ready_to_open()?
        .into_iter()
        .map(MarketId::from)
        .collect())
}

pub fn query_market_ids_ready_to_close(
    postgres: &dyn PostgresInfra,
) -> Result<Vec<MarketId>, failure::Error> {
    Ok(postgres
        .query_market_ids_ready_to_close()?
        .into_iter()
        .map(MarketId::from)
        .collect())
}

/// `orders` はソート済みでなければならない
fn build_market(mut market: QueryMarket, orders: Vec<QueryOrder>) -> Market {
    let market_status = market.status.clone();
    let resolved_token_name = market
        .resolved_token_name
        .clone()
        .map(|n| NonEmptyString::from_str(n).unwrap());
    let reward_records = market.reward_records.take().map(|records| {
        records
            .into_iter()
            .map(|record| (UserId::from(record.user_id), Point::from(record.point)))
            .collect::<HashMap<_, _>>()
    });
    let id = MarketId::from(market.id.clone());
    let market_attrs = build_market_attrs(market);

    let market_orders = MarketOrders {
        orders: orders.into_iter().map(build_order).collect(),
    };

    let token_distribution = TokenDistribution::from(&market_attrs.tokens, &market_orders);

    match market_status {
        InfraMarketStatus::Upcoming => Market::Upcoming(UpcomingMarket {
            id,
            attrs: market_attrs,
            orders: market_orders,
            token_distribution,
        }),
        InfraMarketStatus::Open => Market::Open(OpenMarket {
            id,
            attrs: market_attrs,
            orders: market_orders,
            token_distribution,
        }),
        InfraMarketStatus::Closed => Market::Closed(ClosedMarket {
            id,
            attrs: market_attrs,
            orders: market_orders,
            token_distribution,
        }),
        InfraMarketStatus::Resolved => Market::Resolved(ResolvedMarket {
            id,
            attrs: market_attrs,
            orders: market_orders,
            token_distribution,
            resolved_token_name: resolved_token_name.unwrap(),
            reward_records: RewardRecords(reward_records.unwrap()),
        }),
    }
}

fn build_market_attrs(market: QueryMarket) -> MarketAttrs {
    MarketAttrs {
        title: NonEmptyString::from_str(market.title).unwrap(),
        organizer_id: OrganizerId::from(market.organizer_id),
        description: market.description,
        lmsr_b: lmsr::B::from(market.lmsr_b as u32),
        open: market.open,
        close: market.close,
        tokens: NonEmptyVec::from_vec(
            market
                .tokens
                .into_iter()
                .map(build_market_token)
                .collect::<Vec<_>>(),
        )
        .unwrap(),
    }
}

fn build_market_token(token: QueryToken) -> MarketToken {
    MarketToken {
        name: NonEmptyString::from_str(token.name).unwrap(),
        description: token.description,
        thumbnail_url: token.thumbnail_url,
    }
}

fn convert_marktet_status_to_infra(s: MarketStatus) -> InfraMarketStatus {
    match s {
        MarketStatus::Upcoming => InfraMarketStatus::Upcoming,
        MarketStatus::Open => InfraMarketStatus::Open,
        MarketStatus::Closed => InfraMarketStatus::Closed,
        MarketStatus::Resolved => InfraMarketStatus::Resolved,
    }
}

fn build_order(order: QueryOrder) -> Order {
    match order.type_ {
        InfraOrderType::CoinSupply => Order::from(CoinSupplyOrder::from((
            OrderId::from(order.local_id),
            UserId::from(order.user_id),
            AmountCoin::from(order.amount_coin),
            order.time,
        ))),
        InfraOrderType::Normal => Order::from(NormalOrder::from((
            OrderId::from(order.local_id),
            UserId::from(order.user_id),
            NonEmptyString::from_str(order.token_name.unwrap()).unwrap(),
            AmountToken::from(order.amount_token),
            AmountCoin::from(order.amount_coin),
            order.time,
        ))),
        InfraOrderType::Reward => Order::from(RewardOrder::from((
            OrderId::from(order.local_id),
            UserId::from(order.user_id),
            NonEmptyString::from_str(order.token_name.unwrap()).unwrap(),
            AmountCoin::from(order.amount_coin),
            order.time,
        ))),
    }
}
