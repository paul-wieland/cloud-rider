use crate::mavlink::mavlink_worker::MavlinkWorker;
use crate::websocket::message_channel::MessageChannel;
use crate::websocket::websocket_sender::WebsocketSender;
use axum::extract::ws::{Message, WebSocket};
use futures_util::StreamExt;
use std::time::Duration;

pub struct ConnectionManager {
    socket_sender: Option<WebsocketSender>,
    telemetry_channel: MessageChannel,
    command_channel: MessageChannel,
    mavlink_worker: Option<MavlinkWorker>,
}

impl ConnectionManager {
    pub fn new(socket: WebSocket) -> Self {
        let (tx_socket, rx_socket) = socket.split();

        let (tx_telemetry_channel, rx_telemetry_channel) =
            tokio::sync::mpsc::channel::<Message>(100);

        let (tx_command_channel, rx_command_channel) = tokio::sync::mpsc::channel::<Message>(100);

        let mavlink_worker = MavlinkWorker::new(
            MessageChannel::new(tx_telemetry_channel.clone()),
            rx_command_channel,
            String::from("udpin:0.0.0.0:14550"),
        );

        Self {
            socket_sender: Some(WebsocketSender::new(tx_socket, rx_telemetry_channel)),
            telemetry_channel: MessageChannel::new(tx_telemetry_channel),
            command_channel: MessageChannel::new(tx_command_channel),
            mavlink_worker: Some(mavlink_worker),
        }
    }

    pub async fn run(&mut self) {
        println!("Client started");

        let mut socket_sender = self.socket_sender.take().unwrap();
        let socket_sender_handle = tokio::spawn(async move {
            socket_sender.run().await;
        });

        let mut mavlink_worker = self.mavlink_worker.take().unwrap();
        let mavlink_worker_handle = tokio::spawn(async move {
            mavlink_worker.run().await;
        });

        let heartbeat_producer = self.command_channel.clone();
        let heartbeat_producer_handle = tokio::spawn(async move {
            loop {
                heartbeat_producer.send(Message::text("heartbeat")).await;
                tokio::time::sleep(Duration::from_millis(2000)).await;
            }
        });

        tokio::join!(
            socket_sender_handle,
            mavlink_worker_handle,
            heartbeat_producer_handle
        );
        println!("Client disconnected");
    }
}
