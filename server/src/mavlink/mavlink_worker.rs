use crate::mavlink::mavlink_message_handler::handle_incoming_mavlink_message;
use crate::messages::global_position::{
    from_battery_status, from_global_position_int, GlobalPosition,
};
use crate::websocket::message_channel::MessageChannel;
use axum::extract::ws::Message;
use chrono::Utc;
use mavlink::common::MavAutopilot::MAV_AUTOPILOT_INVALID;
use mavlink::common::MavState::MAV_STATE_ACTIVE;
use mavlink::common::MavType::MAV_TYPE_GCS;
use mavlink::common::{MavMessage, MavModeFlag, HEARTBEAT_DATA};
use mavlink::{connect, MavConnection, MavHeader};
use serde_json::json;
use std::sync::Arc;
use std::thread;
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;

pub struct MavlinkWorker {
    telemetry_channel_sender: MessageChannel,
    command_channel_receiver: Option<Receiver<Message>>,
    connection_string: String,
}

impl MavlinkWorker {
    pub fn new(
        telemetry_channel_sender: MessageChannel,
        command_channel: Receiver<Message>,
        connection_string: String,
    ) -> Self {
        Self {
            telemetry_channel_sender,
            command_channel_receiver: Some(command_channel),
            connection_string,
        }
    }

    pub async fn run(&mut self) {
        let mavlink_connection = Arc::new(connect::<MavMessage>(&self.connection_string).unwrap());

        let command_channel_receiver = self.command_channel_receiver.take().unwrap();
        let mavlink_sender_handle =
            self.run_mavlink_sender(mavlink_connection.clone(), command_channel_receiver);

        let mavlink_receiver_handle = self.run_mavlink_receiver(
            mavlink_connection.clone(),
            self.telemetry_channel_sender.clone(),
        );

        tokio::join!(mavlink_sender_handle, mavlink_receiver_handle);
    }

    async fn run_mavlink_receiver(
        &self,
        mavlink_connection: Arc<Box<dyn MavConnection<MavMessage> + Sync + Send>>,
        telemetry_channel_sender: MessageChannel,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                match mavlink_connection.recv() {
                    Ok((_header, msg)) => {
                        handle_incoming_mavlink_message(msg, &telemetry_channel_sender).await;
                    }
                    Err(e) => eprintln!("Recv error: {}", e),
                }
            }
        })
    }

    async fn run_mavlink_sender(
        &self,
        mavlink_connection: Arc<Box<dyn MavConnection<MavMessage> + Sync + Send>>,
        mut command_channel_receiver: Receiver<Message>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            while let Some(message) = command_channel_receiver.recv().await {
                let header = MavHeader {
                    system_id: 1,
                    component_id: 2,
                    sequence: 0,
                };

                let heartbeat = MavMessage::HEARTBEAT(HEARTBEAT_DATA {
                    custom_mode: 0,
                    mavtype: MAV_TYPE_GCS,
                    autopilot: MAV_AUTOPILOT_INVALID,
                    base_mode: MavModeFlag::MAV_MODE_FLAG_CUSTOM_MODE_ENABLED,
                    system_status: MAV_STATE_ACTIVE,
                    mavlink_version: 3,
                });
                mavlink_connection
                    .send(&header, &heartbeat)
                    .expect("TODO: panic message");
            }
        })
    }
}
