use crate::domain::cloud_rider_message::CloudRiderMessage;
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct MessageChannel {
    tx_sending_channel: Sender<CloudRiderMessage>,
}

impl MessageChannel {
    pub fn new(tx_sending_channel: Sender<CloudRiderMessage>) -> Self {
        Self { tx_sending_channel }
    }

    pub async fn send(&self, message: CloudRiderMessage) {
        self.tx_sending_channel.send(message).await.unwrap();
    }
}
