use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "client")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "client", wasm_bindgen)]
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Params {
    market_id: Uuid,
    account_id: Uuid,
    outcome_id: Uuid,
}

#[cfg(feature = "server")]
impl Params {
    pub fn market_id(&self) -> Uuid {
        self.market_id
    }

    pub fn account_id(&self) -> Uuid {
        self.market_id
    }

    pub fn outcome_id(&self) -> Uuid {
        self.market_id
    }
}

#[cfg_attr(feature = "client", wasm_bindgen)]
#[cfg(feature = "client")]
impl Params {
    #[wasm_bindgen(constructor)]
    pub fn new(market_id: String, account_id: String, outcome_id: String) -> Params {
        Params {
            market_id: Uuid::parse_str(market_id.as_str()).unwrap(),
            account_id: Uuid::parse_str(account_id.as_str()).unwrap(),
            outcome_id: Uuid::parse_str(outcome_id.as_str()).unwrap(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn market_id(&self) -> String {
        self.market_id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn account_id(&self) -> String {
        self.market_id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn outcome_id(&self) -> String {
        self.market_id.to_string()
    }
}

#[cfg_attr(feature = "client", wasm_bindgen)]
#[derive(Debug, Serialize)]
pub struct Success();

#[cfg_attr(feature = "client", wasm_bindgen)]
#[derive(Debug, Serialize)]
pub struct Failure();
