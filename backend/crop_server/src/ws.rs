pub mod msg;

use self::msg::IncomingMsg;
use crate::state;
use crop_domain::{
    account::model::AccountId,
    market::model::{MarketId, OutcomeId},
};
use futures::{future, stream::TryStreamExt as _};
use std::convert::TryFrom as _;
use uuid::Uuid;
use warp::{filters, reject::Rejection, reply::Reply, Filter};

pub fn filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    filters::path::path("stream")
        .and(filters::path::param::<Uuid>())
        .and(filters::path::end())
        .and(filters::ws::ws())
        .map(upgrade_ws)
}

fn upgrade_ws(market_id: Uuid, ws: filters::ws::Ws) -> impl Reply {
    ws.on_upgrade(move |ws| handle_ws(market_id, ws))
}

async fn handle_ws(market_id: Uuid, ws: filters::ws::WebSocket) {
    let market_id = MarketId(market_id);
    ws.try_for_each(|msg| match IncomingMsg::try_from(&msg) {
        Ok(IncomingMsg::Vote(vote)) => {
            let account_id = AccountId(vote.account_id);
            let outcome_id = OutcomeId(vote.outcome_id);
            state::with_market(market_id, |market| {
                market.new_order(account_id, outcome_id);
            });
            future::ok(())
        }
        Err(e) => {
            log::debug!("{:?}", e);
            future::ok(())
        }
    })
    .await
    .unwrap_or_else(|e| log::debug!("{:?}", e))
}
