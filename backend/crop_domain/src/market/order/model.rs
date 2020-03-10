use crate::account::model::AccountName;
use crate::market::model::{num::TipNum, OutcomeId};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// 対象のOutcomeのShareを1つ買うOrderを表現するモデル
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub id: OrderId,
    pub time: DateTime<Utc>,
    pub outcome_id: OutcomeId,
    pub account_name: AccountName,

    /// このOrderに必要なTipの量( >0 )
    pub tip_cost: TipNum,
}

impl Order {
    pub(in crate::market) fn new(
        outcome: OutcomeId,
        account: AccountName,
        tip_cost: TipNum,
    ) -> Order {
        Order {
            id: OrderId::new(),
            time: Utc::now(),
            outcome_id: outcome,
            account_name: account,
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
