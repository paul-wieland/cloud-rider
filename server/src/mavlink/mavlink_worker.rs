use crate::domain::cloud_rider_message::CloudRiderMessage;
use crate::domain::message_channel::MessageChannel;
use crate::mavlink::mavlink_message_mapper::{to_cloud_rider_message, to_mavlink};
use mavlink::common::MavMessage;
use mavlink::{connect, MavConnection, MavHeader};
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;

pub struct MavlinkWorker {
    telemetry_channel_sender: MessageChannel,
    command_channel_receiver: Option<Receiver<CloudRiderMessage>>,
    connection_string: String,
}

impl MavlinkWorker {
    pub fn new(
        telemetry_channel_sender: MessageChannel,
        command_channel: Receiver<CloudRiderMessage>,
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
        // Use blocking task as mavlink_connection.recv() is blocking
        tokio::task::spawn_blocking(async move {
            loop {
                match mavlink_connection.recv() {
                    Ok((_header, message)) => {
                        if let Some(mapped_message) = to_cloud_rider_message(&message) {
                            telemetry_channel_sender.send(mapped_message).await
                        }
                    }
                    Err(e) => eprintln!("Recv error: {}", e),
                }
            }
        })
    }

    async fn run_mavlink_sender(
        &self,
        mavlink_connection: Arc<Box<dyn MavConnection<MavMessage> + Sync + Send>>,
        mut command_channel_receiver: Receiver<CloudRiderMessage>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            while let Some(message) = command_channel_receiver.recv().await {
                let mapped_message = to_mavlink(&message);

                let header = MavHeader {
                    system_id: 1,
                    component_id: 2,
                    sequence: 0,
                };

                match mapped_message {
                    None => {}
                    Some(mavlink_message) => {
                        mavlink_connection
                            .send(&header, &mavlink_message)
                            .expect("TODO: panic message");
                    }
                }
            }
        })
    }
}
