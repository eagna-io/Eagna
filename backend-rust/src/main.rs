use librohan::{app::ApiServer, infra::store::DbStoreFactory};

fn main() {
    let store_factory = DbStoreFactory::new_with_env();
    let bind = std::env::var("BIND").expect("BIND is not specified");
    ApiServer::new(store_factory).run(bind.as_str());
}
