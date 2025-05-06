use crate::domain::cloud_rider_message::CloudRiderMessage;
use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use tokio::sync::mpsc::Receiver;

pub struct WebsocketSender {
    tx_socket: SplitSink<WebSocket, Message>,
    rx_sending_channel: Receiver<CloudRiderMessage>,
}

impl WebsocketSender {
    pub fn new(
        tx_socket: SplitSink<WebSocket, Message>,
        rx_sending_channel: Receiver<CloudRiderMessage>,
    ) -> WebsocketSender {
        WebsocketSender {
            tx_socket,
            rx_sending_channel,
        }
    }

    pub async fn run(&mut self) {
        while let Some(message) = self.rx_sending_channel.recv().await {
            let json_message = serde_json::to_string(&message).unwrap();
            if let Err(e) = self.tx_socket.send(Message::text(json_message)).await {
                eprintln!("error: {}", e);
                break;
            }
        }
    }
}
