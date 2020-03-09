pub mod market;

use self::market::MarketManager;
use crop_domain::market::model::{Market, MarketId};
use std::collections::HashMap;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref GLOBAL_STATE: Context = Context {
        markets: Mutex::new(HashMap::new())
    };
}

pub struct Context {
    markets: Mutex<HashMap<MarketId, MarketManager>>,
}

impl Context {
    pub async fn add_new_market(&self, market: Market) {
        self.markets
            .lock()
            .await
            .insert(market.id.clone(), MarketManager::new(market));
    }

    pub async fn get_market_state(&self, market_id: MarketId) -> Option<MarketManager> {
        self.markets.lock().await.get(&market_id).cloned()
    }
}

pub async fn add_new_market(market: Market) {
    GLOBAL_STATE.add_new_market(market).await
}

pub async fn get_market_state(market_id: MarketId) -> Option<MarketManager> {
    GLOBAL_STATE.get_market_state(market_id).await
}
