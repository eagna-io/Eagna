use librohan::{app::ApiServer, infra::store::DbStoreFactory};

fn main() {
    let store_factory = DbStoreFactory::new_with_env();
    ApiServer::new(store_factory).run("localhost:8088");
}
