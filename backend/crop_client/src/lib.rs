use futures::{future, sink::SinkExt as _, stream};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, Message},
};

pub async fn connect(req: impl IntoClientRequest + Unpin) {
    let (mut ws, _) = connect_async(req).await.unwrap();
    let msg = Message::Text("Hello World".to_string());
    ws.send_all(&mut stream::once(future::ok(msg)))
        .await
        .unwrap();
}
