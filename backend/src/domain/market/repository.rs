use super::*;
use crate::domain::{organizer::OrganizerId, point::Point};
use crate::infra::postgres::{
    market::*,
    types::{MarketStatus as InfraMarketStatus, OrderType as InfraOrderType},
    PostgresInfra,
};
use arrayvec::ArrayVec;

#[derive(From)]
/// `MarketRepository` の生成には `MarketRepository::from` を使う.
/// `new` メソッドを提供しないのは、リポジトリのモデル的なライフサイクルを明確にするため。
/// つまり、リポジトリはモデル的にはプログラムの実行前から実行後までずっと存在する。
/// よってプログラム上では新規作成するのではなく、再構築するという表現の方が正しい。
pub struct MarketRepository<'a> {
    postgres: &'a dyn PostgresInfra,
}

impl<'a> MarketRepository<'a> {
    // RepositoryにMarketを記録する
    // まだ存在しない場合は新しく記録する。
    // すでに存在する場合は新しいものに更新する。
    // ただし、実際には「すでに存在しているかどうか」はチェックしていない。
    // この構造体はモデルに関する知識を持っているので、そのチェックを別の方法で代替できる。
    // 例えば、Upcomingマーケットをsaveするとき、それは必ず「作製」のsaveを意味する。
    // 逆に、「作製」のsaveはこの時以外ありえない。
    // 詳細はコードを参照のこと。
    pub fn save_market(&self, market: &Market) -> Result<(), failure::Error> {
        match market {
            Market::Upcoming(ref m) => self.save_upcoming_market(m),
            Market::Open(ref m) => self.save_open_market(m),
            Market::Closed(ref m) => self.save_closed_market(m),
            Market::Resolved(ref m) => self.save_resolved_market(m),
        }
    }

    fn save_upcoming_market(&self, market: &UpcomingMarket) -> Result<(), failure::Error> {
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
        let mut new_prizes = market
            .attrs()
            .prizes
            .iter()
            .enumerate()
            .map(|(i, prize)| NewPrize {
                local_id: i as i32,
                name: prize.name.as_str(),
                thumbnail_url: prize.thumbnail_url.as_str(),
                target: prize.target.as_str(),
            });
        let new_market = NewMarket {
            id: market.id().as_uuid(),
            title: market.attrs().title.as_str(),
            organizer_id: market.attrs().organizer_id.as_uuid(),
            description: market.attrs().description.as_str(),
            lmsr_b: market.attrs().lmsr_b.to_u32() as i32,
            total_reward_point: market.attrs().total_reward_point.as_u32(),
            open: market.attrs().open.as_date_time(),
            close: market.attrs().close.as_date_time(),
            tokens: &mut new_tokens,
            prizes: &mut new_prizes,
        };

        self.postgres.insert_upcoming_market(new_market)
    }

    fn save_open_market(&self, market: &OpenMarket) -> Result<(), failure::Error> {
        match market.orders().last_order() {
            None => {
                // 単純に status を open に変えるだけ
                self.postgres
                    .update_market_status(market.id().as_uuid(), &InfraMarketStatus::Open)
            }
            Some(order) => {
                // 最も新しい order だけ記録する
                let new_order = NewOrder {
                    local_id: order.id().as_i32(),
                    user_id: order.user_id().as_str(),
                    token_name: order.token_name().map(|tn| tn.as_str()),
                    amount_token: order.amount_token().as_i32(),
                    amount_coin: order.amount_coin().as_i32(),
                    type_: order.type_().as_infra(),
                    time: *order.time(),
                };
                self.postgres
                    .insert_orders(market.id().as_uuid(), &mut std::iter::once(new_order))
            }
        }
    }

    fn save_closed_market(&self, market: &ClosedMarket) -> Result<(), failure::Error> {
        self.postgres
            .update_market_status(market.id().as_uuid(), &InfraMarketStatus::Closed)
    }

    fn save_resolved_market(&self, market: &ResolvedMarket) -> Result<(), failure::Error> {
        // status を resolved に変更し、resolved_token_name を設定する
        self.postgres.update_market_status_and_resolved_token_name(
            market.id().as_uuid(),
            &InfraMarketStatus::Resolved,
            market.resolved_token_name().unwrap().as_str(),
        )?;
        // RewardOrder を記録する
        let mut reward_orders = market.orders().filter_reward_orders().map(|o| NewOrder {
            local_id: o.id.as_i32(),
            user_id: o.user_id.as_str(),
            token_name: Some(o.token_name.as_str()),
            amount_token: 0,
            amount_coin: o.amount_coin.as_i32(),
            type_: InfraOrderType::Reward,
            time: o.time,
        });
        self.postgres
            .insert_orders(market.id().as_uuid(), &mut reward_orders)
    }

    pub fn lock_market(&self, market_id: &MarketId) -> Result<(), failure::Error> {
        self.postgres.lock_market(market_id.as_uuid())
    }

    pub fn query_market(&self, market_id: &MarketId) -> Result<Option<Market>, failure::Error> {
        let raw_market = match self.postgres.query_market_by_id(market_id.as_uuid())? {
            None => return Ok(None),
            Some(m) => m,
        };
        let raw_orders = self
            .postgres
            .query_orders_by_market_id(market_id.as_uuid())?;
        Ok(Some(build_market(raw_market, raw_orders)))
    }

    pub fn query_markets(&self, market_ids: &[MarketId]) -> Result<Vec<Market>, failure::Error> {
        let mut markets = Vec::with_capacity(market_ids.len());
        // TODO
        // Parallelに実行できるようにする
        for market_id in market_ids {
            if let Some(market) = self.query_market(market_id)? {
                markets.push(market);
            }
        }
        Ok(markets)
    }

    pub fn query_market_ids_with_status(
        &self,
        statuses: &[MarketStatus],
    ) -> Result<Vec<MarketId>, failure::Error> {
        let mut infra_statuses = ArrayVec::<[InfraMarketStatus; 4]>::new();
        for status in statuses {
            let _ = infra_statuses.try_push((*status).into());
        }

        Ok(self
            .postgres
            .query_market_ids_by_status(infra_statuses.as_slice())?
            .into_iter()
            .map(MarketId::from)
            .collect())
    }

    pub fn query_market_ids_participated_by_user(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<MarketId>, failure::Error> {
        Ok(self
            .postgres
            .query_market_ids_participated_by_user(user_id.as_str())?
            .into_iter()
            .map(MarketId::from)
            .collect())
    }

    pub fn query_market_ids_ready_to_open(&self) -> Result<Vec<MarketId>, failure::Error> {
        Ok(self
            .postgres
            .query_market_ids_ready_to_open()?
            .into_iter()
            .map(MarketId::from)
            .collect())
    }

    pub fn query_market_ids_ready_to_close(&self) -> Result<Vec<MarketId>, failure::Error> {
        Ok(self
            .postgres
            .query_market_ids_ready_to_close()?
            .into_iter()
            .map(MarketId::from)
            .collect())
    }
}

/// `orders` はソート済みでなければならない
fn build_market(market: QueryMarket, orders: Vec<QueryOrder>) -> Market {
    let market_status = market.status.clone();
    let resolved_token_name = market.resolved_token_name.clone().map(|n| TokenName(n));
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
        }),
    }
}

fn build_market_attrs(market: QueryMarket) -> MarketAttrs {
    MarketAttrs {
        title: MarketTitle::from(market.title),
        organizer_id: OrganizerId::from(market.organizer_id),
        description: MarketDesc::from(market.description),
        lmsr_b: lmsr::B::from(market.lmsr_b as u32),
        total_reward_point: Point::from(market.total_reward_point as u32),
        open: MarketOpenTime::from(market.open),
        close: MarketCloseTime::from(market.close),
        tokens: MarketTokens::from(
            market
                .tokens
                .into_iter()
                .map(build_market_token)
                .collect::<Vec<_>>(),
        ),
        prizes: MarketPrizes::from(
            market
                .prizes
                .into_iter()
                .map(build_market_prize)
                .collect::<Vec<_>>(),
        ),
    }
}

fn build_market_token(token: QueryToken) -> Token {
    Token {
        name: TokenName(token.name),
        description: TokenDesc(token.description),
        thumbnail_url: TokenThumbnailUrl(token.thumbnail_url),
    }
}

fn build_market_prize(prize: QueryPrize) -> Prize {
    Prize {
        id: PrizeId(prize.local_id),
        name: PrizeName(prize.name),
        thumbnail_url: PrizeThumbnailUrl(prize.thumbnail_url),
        target: PrizeTarget(prize.target),
    }
}

fn build_order(order: QueryOrder) -> Order {
    match order.type_ {
        InfraOrderType::CoinSupply => Order::from(CoinSupplyOrder {
            id: OrderId::from(order.local_id),
            user_id: UserId::from_str(order.user_id.as_str()),
            amount_coin: AmountCoin::from(order.amount_coin),
            time: order.time,
        }),
        InfraOrderType::Normal => Order::from(NormalOrder {
            id: OrderId::from(order.local_id),
            user_id: UserId::from_str(order.user_id.as_str()),
            token_name: TokenName::from(order.token_name.unwrap()),
            amount_token: AmountToken::from(order.amount_token),
            amount_coin: AmountCoin::from(order.amount_coin),
            time: order.time,
        }),
        InfraOrderType::Reward => Order::from(RewardOrder {
            id: OrderId::from(order.local_id),
            user_id: UserId::from_str(order.user_id.as_str()),
            token_name: TokenName::from(order.token_name.unwrap()),
            amount_coin: AmountCoin::from(order.amount_coin),
            time: order.time,
        }),
    }
}
