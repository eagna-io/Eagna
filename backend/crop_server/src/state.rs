pub mod market;

use self::market::State as MarketState;
use crop_domain::market::model::{Market, MarketId};
use std::collections::HashMap;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref GLOBAL_STATE: State = State {
        markets: Mutex::new(HashMap::new())
    };
}

pub struct State {
    markets: Mutex<HashMap<MarketId, MarketState>>,
}

impl State {
    pub async fn add_new_market(&self, market: Market) {
        self.markets
            .lock()
            .await
            .insert(market.id.clone(), MarketState::new(market));
    }

    pub async fn get_market_state(&self, market_id: MarketId) -> Option<MarketState> {
        self.markets.lock().await.get(&market_id).cloned()
    }
}

pub async fn add_new_market(market: Market) {
    GLOBAL_STATE.add_new_market(market).await
}

pub async fn get_market_state(market_id: MarketId) -> Option<MarketState> {
    GLOBAL_STATE.get_market_state(market_id).await
}
