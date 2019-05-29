use librohan::{app::ApiServer, infra::store::DbStoreFactory};
use log::info;

fn main() {
    env_logger::init();

    let store_factory = DbStoreFactory::new_with_env();
    let bind = std::env::var("BIND").expect("BIND is not specified");

    info!("Server is starting on {}", bind.as_str());

    ApiServer::new(store_factory, "*").run(bind.as_str());
}
