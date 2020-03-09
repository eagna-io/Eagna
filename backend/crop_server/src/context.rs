pub mod market;

use self::market::MarketManager;
use crop_domain::market::model::{Market, MarketId};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Context {
    markets: Arc<Mutex<HashMap<MarketId, MarketManager>>>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            markets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

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
