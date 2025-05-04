use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use tokio::sync::mpsc::Receiver;

pub struct WebsocketSender {
    tx_socket: SplitSink<WebSocket, Message>,
    rx_sending_channel: Receiver<Message>,
}

impl WebsocketSender {
    pub fn new(
        tx_socket: SplitSink<WebSocket, Message>,
        rx_sending_channel: Receiver<Message>,
    ) -> WebsocketSender {
        WebsocketSender {
            tx_socket,
            rx_sending_channel,
        }
    }

    pub async fn run(&mut self) {
        while let Some(message) = self.rx_sending_channel.recv().await {
            if let Err(e) = self.tx_socket.send(message).await {
                eprintln!("error: {}", e);
                break;
            }
        }
    }
}
