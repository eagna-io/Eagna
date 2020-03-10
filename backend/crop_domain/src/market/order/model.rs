use crate::account::model::AccountId;
use crate::market::model::{num::TipNum, OutcomeId};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// 対象のOutcomeのShareを1つ買うOrderを表現するモデル
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Order {
    pub id: OrderId,
    pub time: DateTime<Utc>,
    pub outcome_id: OutcomeId,
    pub account_id: AccountId,

    /// このOrderに必要なTipの量( >0 )
    pub tip_cost: TipNum,
}

impl Order {
    pub(in crate::market) fn new(
        outcome: OutcomeId,
        account: AccountId,
        tip_cost: TipNum,
    ) -> Order {
        Order {
            id: OrderId::new(),
            time: Utc::now(),
            outcome_id: outcome,
            account_id: account,
            tip_cost,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderId(Uuid);

impl OrderId {
    fn new() -> OrderId {
        OrderId(Uuid::new_v4())
    }
}
