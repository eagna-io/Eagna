use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Params {
    market_id: Uuid,
    account_id: Uuid,
    outcome_id: Uuid,
}

#[wasm_bindgen]
#[derive(Debug, Serialize)]
pub struct Success();

#[derive(Debug, Serialize)]
pub struct Failure();
