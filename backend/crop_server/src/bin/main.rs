use crop_server::server::Server;

#[tokio::main]
async fn main() {
    Server::bind(([127, 0, 0, 1], 3030)).await;
}
