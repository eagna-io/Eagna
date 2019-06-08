use chrono::{Duration, Utc};
use librohan::{domain::services::StoreFactory, infra::store::DbStoreFactory};

fn main() {
    let store_factory = DbStoreFactory::new_with_env();
    let mut store = store_factory.establish();

    utils::market::insert_new_market(&mut store, Utc::now() - Duration::minutes(10));
    utils::market::insert_new_market(&mut store, Utc::now());
}
