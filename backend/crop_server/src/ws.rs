pub mod msg;

use self::msg::IncomingMsg;
use crate::state;
use crop_domain::{
    account::model::AccountId,
    market::model::{MarketId, OutcomeId},
};
use futures::stream::TryStreamExt as _;
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

    match state::get_market_state(market_id).await {
        None => log::info!("Market not found"),
        Some(market_state) => ws
            .try_for_each(move |msg| {
                let cloned_state = market_state.clone();
                async move {
                    match IncomingMsg::try_from(&msg) {
                        Ok(IncomingMsg::Vote(vote)) => {
                            let account_id = AccountId(vote.account_id);
                            let outcome_id = OutcomeId(vote.outcome_id);

                            cloned_state.vote(account_id, outcome_id).await;
                            Ok(())
                        }
                        Err(e) => {
                            log::debug!("{:?}", e);
                            Ok(())
                        }
                    }
                }
            })
            .await
            .unwrap_or_else(|e| log::debug!("{:?}", e)),
    }
}
