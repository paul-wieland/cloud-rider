use axum::extract::ws::Message;
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct MessageChannel {
    tx_sending_channel: Sender<Message>,
}

impl MessageChannel {
    pub fn new(tx_sending_channel: Sender<Message>) -> Self {
        Self { tx_sending_channel }
    }

    pub async fn send(&self, message: Message) {
        self.tx_sending_channel.send(message).await.unwrap();
    }
}
