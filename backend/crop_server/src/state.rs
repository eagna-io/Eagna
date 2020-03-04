use crop_domain::market::model::{Market, MarketId};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

lazy_static::lazy_static! {
    pub static ref GLOBAL_STATE: State = State {
        markets: Mutex::new(HashMap::new())
    };
}

pub struct State {
    markets: Mutex<HashMap<MarketId, Arc<Mutex<Market>>>>,
}

impl State {
    pub fn add_new_market(&self, market: Market) {
        self.markets
            .lock()
            .unwrap()
            .insert(market.id.clone(), Arc::new(Mutex::new(market)));
    }

    pub fn with_market<T>(
        &self,
        market_id: MarketId,
        f: impl FnOnce(&mut Market) -> T,
    ) -> Option<T> {
        if let Some(market) = self.get_market(market_id) {
            let mut locked = market.lock().unwrap();
            let t = f(&mut locked);
            Some(t)
        } else {
            None
        }
    }

    fn get_market(&self, market_id: MarketId) -> Option<Arc<Mutex<Market>>> {
        self.markets.lock().unwrap().get(&market_id).cloned()
    }
}

pub fn add_new_market(market: Market) {
    GLOBAL_STATE.add_new_market(market)
}

pub fn with_market<T>(market_id: MarketId, f: impl FnOnce(&mut Market) -> T) -> Option<T> {
    GLOBAL_STATE.with_market(market_id, f)
}
