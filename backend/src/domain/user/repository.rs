use super::*;
use crate::domain::{market::MarketId, prize::PrizeId};
use crate::infra::postgres::{
    types::PrizeTradeStatus as InfraPrizeTradeStatus,
    user::{
        NewMarketRewardHistoryItem, NewPointHistoryItem, NewPrizeTradeHistoryItem, NewUser,
        QueryPointHistoryItem,
    },
    PostgresInfra,
};

#[derive(From)]
pub struct UserRepository<'a> {
    postgres: &'a dyn PostgresInfra,
}

impl<'a> UserRepository<'a> {
    /// ユーザーを新規作成したとき、それを infra に記録する。
    /// PointHistory は記録しない。
    pub fn save_new_user(&self, user: &User) -> Result<(), failure::Error> {
        let new_user = NewUser {
            id: user.id.as_str(),
            name: user.name.as_str(),
            email: user.email.as_str(),
        };
        self.postgres.save_user(new_user)
    }

    pub fn save_last_point_history(&self, user: &User) -> Result<(), failure::Error> {
        let last_point_history_item = match user.point_history.as_slice().last() {
            None => {
                return Ok(());
            }
            Some(item) => item,
        };
        let new_history_item = convert_point_history_item_to_infra(last_point_history_item);
        self.postgres
            .save_user_point_history(user.id().as_str(), new_history_item)
    }

    pub fn query_user(&self, user_id: &UserId) -> Result<Option<User>, failure::Error> {
        match self.postgres.query_user(user_id.as_str())? {
            None => Ok(None),
            Some(res) => {
                let point_history: Vec<PointHistoryItem> = self
                    .postgres
                    .query_user_point_history(user_id.as_str())?
                    .into_iter()
                    .map(convert_point_history_item_from_infra)
                    .collect();
                Ok(Some(User {
                    id: *user_id,
                    name: UserName::from_str(res.name)?,
                    email: UserEmail::from_str(res.email)?,
                    is_admin: res.is_admin,
                    point_history: PointHistory::from(point_history),
                }))
            }
        }
    }
}

fn convert_point_history_item_to_infra(item: &PointHistoryItem) -> NewPointHistoryItem {
    match item {
        PointHistoryItem::MarketReward(ref item) => {
            NewPointHistoryItem::MarketReward(NewMarketRewardHistoryItem {
                point: *item.point(),
                time: *item.time(),
                market_id: *item.market_id().as_uuid(),
            })
        }
        PointHistoryItem::PrizeTrade(ref item) => {
            NewPointHistoryItem::PrizeTrade(NewPrizeTradeHistoryItem {
                price: *item.price(),
                time: *item.time(),
                prize_id: *item.prize_id().as_uuid(),
                status: match item.status() {
                    PrizeTradeStatus::Requested => InfraPrizeTradeStatus::Requested,
                    PrizeTradeStatus::Processed => InfraPrizeTradeStatus::Processed,
                },
            })
        }
    }
}

fn convert_point_history_item_from_infra(item: QueryPointHistoryItem) -> PointHistoryItem {
    match item {
        QueryPointHistoryItem::MarketReward(item) => PointHistoryItem::MarketReward(
            MarketRewardHistoryItem::from((item.point, item.time, MarketId::from(item.market_id))),
        ),
        QueryPointHistoryItem::PrizeTrade(item) => {
            PointHistoryItem::PrizeTrade(PrizeTradeHistoryItem::from((
                item.price,
                item.time,
                PrizeId::from(item.prize_id),
                match item.status {
                    InfraPrizeTradeStatus::Requested => PrizeTradeStatus::Requested,
                    InfraPrizeTradeStatus::Processed => PrizeTradeStatus::Processed,
                },
            )))
        }
    }
}
