use super::*;
use crate::infra::postgres::{
    market::*,
    types::{MarketStatus as InfraMarketStatus, OrderType as InfraOrderType},
    PostgresInfra,
};

// RepositoryにMarketを記録する
// まだ存在しない場合は新しく記録する。
// すでに存在する場合は新しいものに更新する。
// ただし、実際には「すでに存在しているかどうか」はチェックしていない。
// この構造体はモデルに関する知識を持っているので、そのチェックを別の方法で代替できる。
// 例えば、Upcomingマーケットをsaveするとき、それは必ず「作製」のsaveを意味する。
// 逆に、「作製」のsaveはこの時以外ありえない。
// 詳細はコードを参照のこと。
pub(super) fn save_market(
    postgres: &dyn PostgresInfra,
    market: &Market,
) -> Result<(), failure::Error> {
    match market {
        Market::Upcoming(ref m) => save_upcoming_market(postgres, m),
        Market::Open(ref m) => save_open_market(postgres, m),
        Market::Closed(ref m) => save_closed_market(postgres, m),
        Market::Resolved(ref m) => save_resolved_market(postgres, m),
    }
}

fn save_upcoming_market(
    postgres: &dyn PostgresInfra,
    market: &UpcomingMarket,
) -> Result<(), failure::Error> {
    // marketに保存されている順にTokenを保存する。
    let mut new_tokens = market
        .attrs()
        .tokens
        .iter()
        .enumerate()
        .map(|(idx, token)| NewToken {
            name: token.name.as_str(),
            description: token.description.as_str(),
            thumbnail_url: token.thumbnail_url.as_str(),
            idx: idx as i32,
        });
    let mut new_prizes = market.attrs().prizes.iter().map(|prize| NewPrize {
        local_id: prize.id,
        name: prize.name.as_str(),
        thumbnail_url: prize.thumbnail_url.as_str(),
        target: prize.target().as_str(),
    });
    let new_market = NewMarket {
        id: market.id().as_uuid(),
        title: market.attrs().title.as_str(),
        organizer_id: market.attrs().organizer_id.as_uuid(),
        description: market.attrs().description.as_str(),
        lmsr_b: market.attrs().lmsr_b.as_u32() as i32,
        total_reward_point: market.attrs().total_reward_point.as_u32(),
        open: market.attrs().open(),
        close: market.attrs().close(),
        tokens: &mut new_tokens,
        prizes: &mut new_prizes,
    };

    postgres.insert_upcoming_market(new_market)
}

fn save_open_market(
    postgres: &dyn PostgresInfra,
    market: &OpenMarket,
) -> Result<(), failure::Error> {
    match market.orders().last_order() {
        None => {
            // 単純に status を open に変えるだけ
            postgres.update_market_status(market.id().as_uuid(), &InfraMarketStatus::Open)
        }
        Some(order) => {
            // 最も新しい order だけ記録する
            let new_order = NewOrder {
                local_id: order.id().as_i32(),
                user_id: order.user_id().as_str(),
                token_name: order.token_name().map(|tn| tn.as_str()),
                amount_token: order.amount_token().as_i32(),
                amount_coin: order.amount_coin().as_i32(),
                type_: convert_order_type_to_infra(order.type_()),
                time: *order.time(),
            };
            postgres.insert_orders(market.id().as_uuid(), &mut std::iter::once(new_order))
        }
    }
}

fn save_closed_market(
    postgres: &dyn PostgresInfra,
    market: &ClosedMarket,
) -> Result<(), failure::Error> {
    postgres.update_market_status(market.id().as_uuid(), &InfraMarketStatus::Closed)
}

fn save_resolved_market(
    postgres: &dyn PostgresInfra,
    market: &ResolvedMarket,
) -> Result<(), failure::Error> {
    // status を resolved に変更し、resolved_token_name を設定する
    postgres.resolve_market(market.id().as_uuid(), market.resolved_token_name().as_str())?;

    // RewardOrder を記録する
    let mut reward_orders = market.orders().filter_reward_orders().map(|o| NewOrder {
        local_id: o.id().as_i32(),
        user_id: o.user_id().as_str(),
        token_name: Some(o.token_name().as_str()),
        amount_token: 0,
        amount_coin: o.amount_coin().as_i32(),
        type_: InfraOrderType::Reward,
        time: *o.time(),
    });
    postgres.insert_orders(market.id().as_uuid(), &mut reward_orders)?;

    // RewardRecord を記録する
    let mut reward_records = market
        .reward_records
        .iter()
        .map(|(user_id, point)| NewRewardRecord {
            user_id: user_id.as_str(),
            point: point.as_u32() as i32,
        });
    postgres.insert_reward_records(*market.id().as_uuid(), &mut reward_records)
}

fn convert_order_type_to_infra(s: OrderType) -> InfraOrderType {
    match s {
        OrderType::CoinSupply => InfraOrderType::CoinSupply,
        OrderType::Normal => InfraOrderType::Normal,
        OrderType::Reward => InfraOrderType::Reward,
    }
}
