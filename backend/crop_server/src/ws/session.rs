use crate::state::market::State as MarketState;
use crate::ws::msg::IncomingMsg;
use crop_domain::{account::model::AccountId, market::model::OutcomeId};
use futures::{future, stream::TryStreamExt as _};
use std::convert::TryFrom as _;
use warp::filters::ws::WebSocket;

pub struct Session {
    market: MarketState,
    ws: WebSocket,
}

impl Session {
    pub fn new(market: MarketState, ws: WebSocket) -> Session {
        Session { market, ws }
    }

    pub async fn handle(self) {
        let Session { market, ws } = self;
        ws.err_into::<anyhow::Error>()
            .and_then(|msg| future::ready(IncomingMsg::try_from(&msg).map_err(anyhow::Error::from)))
            .try_for_each(move |msg| {
                let cloned_market = market.clone();
                async move {
                    match msg {
                        IncomingMsg::Vote(vote) => {
                            let account_id = AccountId(vote.account_id);
                            let outcome_id = OutcomeId(vote.outcome_id);
                            cloned_market.vote(account_id, outcome_id).await;
                            Ok(())
                        }
                    }
                }
            })
            .await
            .unwrap_or_else(|e| log::debug!("{:?}", e))
    }
}
