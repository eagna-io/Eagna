use crop_client::connect;

#[tokio::main]
async fn main() {
    connect("ws://127.0.0.1:3030/stream").await;
}
