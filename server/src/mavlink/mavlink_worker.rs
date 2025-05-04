use crate::messages::global_position::GlobalPosition;
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

pub struct MavlinkWorker {
    telemetry_channel: MessageChannel,
    command_channel: Option<Receiver<Message>>,
    connection_string: String,
}

impl MavlinkWorker {
    pub fn new(
        telemetry_channel: MessageChannel,
        command_channel: Receiver<Message>,
        connection_string: String,
    ) -> Self {
        Self {
            telemetry_channel,
            command_channel: Some(command_channel),
            connection_string,
        }
    }

    pub async fn run(&mut self) {
        let mavlink_connection = Arc::new(connect::<MavMessage>(&self.connection_string).unwrap());

        let mavlink_send_connection = mavlink_connection.clone();
        let mut command_channel = self.command_channel.take().unwrap();
        let mavlink_sender_handle = tokio::spawn(async move {
            while let Some(message) = command_channel.recv().await {
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
                mavlink_send_connection
                    .send(&header, &heartbeat)
                    .expect("TODO: panic message");
            }
        });

        let mavlink_receive_connection = mavlink_connection.clone();
        let telemetry_channel = self.telemetry_channel.clone();
        let mavlink_receiver_handle = tokio::spawn(async move {
            loop {
                match mavlink_receive_connection.recv() {
                    Ok((_header, msg)) => {
                        Self::handle_incoming_mavlink_message(msg, &telemetry_channel).await;
                    }
                    Err(e) => eprintln!("Recv error: {}", e),
                }
            }
        });

        tokio::join!(mavlink_sender_handle, mavlink_receiver_handle);
    }

    async fn handle_incoming_mavlink_message(
        message: MavMessage,
        telemetry_channel: &MessageChannel,
    ) {
        match message {
            MavMessage::HEARTBEAT(_) => {
                let timestamp = Utc::now().to_rfc3339();

                let message = json!({
                    "type": "heartbeat",
                    "data": {
                        "timestamp": timestamp
                    }
                });

                let message_string = message.to_string();
                telemetry_channel
                    .send(Message::text(String::from(message_string)))
                    .await;
            }
            MavMessage::SYS_STATUS(_) => {}
            MavMessage::SYSTEM_TIME(_) => {}
            MavMessage::PING(_) => {}
            MavMessage::CHANGE_OPERATOR_CONTROL(_) => {}
            MavMessage::CHANGE_OPERATOR_CONTROL_ACK(_) => {}
            MavMessage::AUTH_KEY(_) => {}
            MavMessage::LINK_NODE_STATUS(_) => {}
            MavMessage::SET_MODE(_) => {}
            MavMessage::PARAM_REQUEST_READ(_) => {}
            MavMessage::PARAM_REQUEST_LIST(_) => {}
            MavMessage::PARAM_VALUE(_) => {}
            MavMessage::PARAM_SET(_) => {}
            MavMessage::GPS_RAW_INT(_) => {}
            MavMessage::GPS_STATUS(_) => {}
            MavMessage::SCALED_IMU(_) => {}
            MavMessage::RAW_IMU(_) => {}
            MavMessage::RAW_PRESSURE(_) => {}
            MavMessage::SCALED_PRESSURE(_) => {}
            MavMessage::ATTITUDE(_) => {}
            MavMessage::ATTITUDE_QUATERNION(_) => {}
            MavMessage::LOCAL_POSITION_NED(_) => {}
            MavMessage::GLOBAL_POSITION_INT(mavlink_data) => {
                let global_position_data = GlobalPosition::from_global_position_int(mavlink_data);
                let json = serde_json::to_string(&global_position_data).unwrap();
                telemetry_channel.send(Message::text(json)).await;
            }
            MavMessage::RC_CHANNELS_SCALED(_) => {}
            MavMessage::RC_CHANNELS_RAW(_) => {}
            MavMessage::SERVO_OUTPUT_RAW(_) => {}
            MavMessage::MISSION_REQUEST_PARTIAL_LIST(_) => {}
            MavMessage::MISSION_WRITE_PARTIAL_LIST(_) => {}
            MavMessage::MISSION_ITEM(_) => {}
            MavMessage::MISSION_REQUEST(_) => {}
            MavMessage::MISSION_SET_CURRENT(_) => {}
            MavMessage::MISSION_CURRENT(_) => {}
            MavMessage::MISSION_REQUEST_LIST(_) => {}
            MavMessage::MISSION_COUNT(_) => {}
            MavMessage::MISSION_CLEAR_ALL(_) => {}
            MavMessage::MISSION_ITEM_REACHED(_) => {}
            MavMessage::MISSION_ACK(_) => {}
            MavMessage::SET_GPS_GLOBAL_ORIGIN(_) => {}
            MavMessage::GPS_GLOBAL_ORIGIN(_) => {}
            MavMessage::PARAM_MAP_RC(_) => {}
            MavMessage::MISSION_REQUEST_INT(_) => {}
            MavMessage::MISSION_CHANGED(_) => {}
            MavMessage::SAFETY_SET_ALLOWED_AREA(_) => {}
            MavMessage::SAFETY_ALLOWED_AREA(_) => {}
            MavMessage::ATTITUDE_QUATERNION_COV(_) => {}
            MavMessage::NAV_CONTROLLER_OUTPUT(_) => {}
            MavMessage::GLOBAL_POSITION_INT_COV(_) => {}
            MavMessage::LOCAL_POSITION_NED_COV(_) => {}
            MavMessage::RC_CHANNELS(_) => {}
            MavMessage::REQUEST_DATA_STREAM(_) => {}
            MavMessage::DATA_STREAM(_) => {}
            MavMessage::MANUAL_CONTROL(_) => {}
            MavMessage::RC_CHANNELS_OVERRIDE(_) => {}
            MavMessage::MISSION_ITEM_INT(_) => {}
            MavMessage::VFR_HUD(_) => {}
            MavMessage::COMMAND_INT(_) => {}
            MavMessage::COMMAND_LONG(_) => {}
            MavMessage::COMMAND_ACK(_) => {}
            MavMessage::COMMAND_CANCEL(_) => {}
            MavMessage::MANUAL_SETPOINT(_) => {}
            MavMessage::SET_ATTITUDE_TARGET(_) => {}
            MavMessage::ATTITUDE_TARGET(_) => {}
            MavMessage::SET_POSITION_TARGET_LOCAL_NED(_) => {}
            MavMessage::POSITION_TARGET_LOCAL_NED(_) => {}
            MavMessage::SET_POSITION_TARGET_GLOBAL_INT(_) => {}
            MavMessage::POSITION_TARGET_GLOBAL_INT(_) => {}
            MavMessage::LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET(_) => {}
            MavMessage::HIL_STATE(_) => {}
            MavMessage::HIL_CONTROLS(_) => {}
            MavMessage::HIL_RC_INPUTS_RAW(_) => {}
            MavMessage::HIL_ACTUATOR_CONTROLS(_) => {}
            MavMessage::OPTICAL_FLOW(_) => {}
            MavMessage::GLOBAL_VISION_POSITION_ESTIMATE(_) => {}
            MavMessage::VISION_POSITION_ESTIMATE(_) => {}
            MavMessage::VISION_SPEED_ESTIMATE(_) => {}
            MavMessage::VICON_POSITION_ESTIMATE(_) => {}
            MavMessage::HIGHRES_IMU(_) => {}
            MavMessage::OPTICAL_FLOW_RAD(_) => {}
            MavMessage::HIL_SENSOR(_) => {}
            MavMessage::SIM_STATE(_) => {}
            MavMessage::RADIO_STATUS(_) => {}
            MavMessage::FILE_TRANSFER_PROTOCOL(_) => {}
            MavMessage::TIMESYNC(_) => {}
            MavMessage::CAMERA_TRIGGER(_) => {}
            MavMessage::HIL_GPS(_) => {}
            MavMessage::HIL_OPTICAL_FLOW(_) => {}
            MavMessage::HIL_STATE_QUATERNION(_) => {}
            MavMessage::SCALED_IMU2(_) => {}
            MavMessage::LOG_REQUEST_LIST(_) => {}
            MavMessage::LOG_ENTRY(_) => {}
            MavMessage::LOG_REQUEST_DATA(_) => {}
            MavMessage::LOG_DATA(_) => {}
            MavMessage::LOG_ERASE(_) => {}
            MavMessage::LOG_REQUEST_END(_) => {}
            MavMessage::GPS_INJECT_DATA(_) => {}
            MavMessage::GPS2_RAW(_) => {}
            MavMessage::POWER_STATUS(_) => {}
            MavMessage::SERIAL_CONTROL(_) => {}
            MavMessage::GPS_RTK(_) => {}
            MavMessage::GPS2_RTK(_) => {}
            MavMessage::SCALED_IMU3(_) => {}
            MavMessage::DATA_TRANSMISSION_HANDSHAKE(_) => {}
            MavMessage::ENCAPSULATED_DATA(_) => {}
            MavMessage::DISTANCE_SENSOR(_) => {}
            MavMessage::TERRAIN_REQUEST(_) => {}
            MavMessage::TERRAIN_DATA(_) => {}
            MavMessage::TERRAIN_CHECK(_) => {}
            MavMessage::TERRAIN_REPORT(_) => {}
            MavMessage::SCALED_PRESSURE2(_) => {}
            MavMessage::ATT_POS_MOCAP(_) => {}
            MavMessage::SET_ACTUATOR_CONTROL_TARGET(_) => {}
            MavMessage::ACTUATOR_CONTROL_TARGET(_) => {}
            MavMessage::ALTITUDE(_) => {}
            MavMessage::RESOURCE_REQUEST(_) => {}
            MavMessage::SCALED_PRESSURE3(_) => {}
            MavMessage::FOLLOW_TARGET(_) => {}
            MavMessage::CONTROL_SYSTEM_STATE(_) => {}
            MavMessage::BATTERY_STATUS(_) => {}
            MavMessage::AUTOPILOT_VERSION(_) => {}
            MavMessage::LANDING_TARGET(_) => {}
            MavMessage::FENCE_STATUS(_) => {}
            MavMessage::ESTIMATOR_STATUS(_) => {}
            MavMessage::WIND_COV(_) => {}
            MavMessage::GPS_INPUT(_) => {}
            MavMessage::GPS_RTCM_DATA(_) => {}
            MavMessage::HIGH_LATENCY(_) => {}
            MavMessage::HIGH_LATENCY2(_) => {}
            MavMessage::VIBRATION(_) => {}
            MavMessage::HOME_POSITION(_) => {}
            MavMessage::SET_HOME_POSITION(_) => {}
            MavMessage::MESSAGE_INTERVAL(_) => {}
            MavMessage::EXTENDED_SYS_STATE(_) => {}
            MavMessage::ADSB_VEHICLE(_) => {}
            MavMessage::COLLISION(_) => {}
            MavMessage::V2_EXTENSION(_) => {}
            MavMessage::MEMORY_VECT(_) => {}
            MavMessage::DEBUG_VECT(_) => {}
            MavMessage::NAMED_VALUE_FLOAT(_) => {}
            MavMessage::NAMED_VALUE_INT(_) => {}
            MavMessage::STATUSTEXT(_) => {}
            MavMessage::DEBUG(_) => {}
            MavMessage::SETUP_SIGNING(_) => {}
            MavMessage::BUTTON_CHANGE(_) => {}
            MavMessage::PLAY_TUNE(_) => {}
            MavMessage::CAMERA_INFORMATION(_) => {}
            MavMessage::CAMERA_SETTINGS(_) => {}
            MavMessage::STORAGE_INFORMATION(_) => {}
            MavMessage::CAMERA_CAPTURE_STATUS(_) => {}
            MavMessage::CAMERA_IMAGE_CAPTURED(_) => {}
            MavMessage::FLIGHT_INFORMATION(_) => {}
            MavMessage::MOUNT_ORIENTATION(_) => {}
            MavMessage::LOGGING_DATA(_) => {}
            MavMessage::LOGGING_DATA_ACKED(_) => {}
            MavMessage::LOGGING_ACK(_) => {}
            MavMessage::VIDEO_STREAM_INFORMATION(_) => {}
            MavMessage::VIDEO_STREAM_STATUS(_) => {}
            MavMessage::GIMBAL_MANAGER_INFORMATION(_) => {}
            MavMessage::GIMBAL_MANAGER_STATUS(_) => {}
            MavMessage::GIMBAL_MANAGER_SET_ATTITUDE(_) => {}
            MavMessage::GIMBAL_DEVICE_INFORMATION(_) => {}
            MavMessage::GIMBAL_DEVICE_SET_ATTITUDE(_) => {}
            MavMessage::GIMBAL_DEVICE_ATTITUDE_STATUS(_) => {}
            MavMessage::AUTOPILOT_STATE_FOR_GIMBAL_DEVICE(_) => {}
            MavMessage::GIMBAL_MANAGER_SET_TILTPAN(_) => {}
            MavMessage::WIFI_CONFIG_AP(_) => {}
            MavMessage::PROTOCOL_VERSION(_) => {}
            MavMessage::AIS_VESSEL(_) => {}
            MavMessage::UAVCAN_NODE_STATUS(_) => {}
            MavMessage::UAVCAN_NODE_INFO(_) => {}
            MavMessage::PARAM_EXT_REQUEST_READ(_) => {}
            MavMessage::PARAM_EXT_REQUEST_LIST(_) => {}
            MavMessage::PARAM_EXT_VALUE(_) => {}
            MavMessage::PARAM_EXT_SET(_) => {}
            MavMessage::PARAM_EXT_ACK(_) => {}
            MavMessage::OBSTACLE_DISTANCE(_) => {}
            MavMessage::ODOMETRY(_) => {}
            MavMessage::TRAJECTORY_REPRESENTATION_WAYPOINTS(_) => {}
            MavMessage::TRAJECTORY_REPRESENTATION_BEZIER(_) => {}
            MavMessage::CELLULAR_STATUS(_) => {}
            MavMessage::ISBD_LINK_STATUS(_) => {}
            MavMessage::CELLULAR_CONFIG(_) => {}
            MavMessage::RAW_RPM(_) => {}
            MavMessage::UTM_GLOBAL_POSITION(_) => {}
            MavMessage::DEBUG_FLOAT_ARRAY(_) => {}
            MavMessage::ORBIT_EXECUTION_STATUS(_) => {}
            MavMessage::SMART_BATTERY_INFO(_) => {}
            MavMessage::SMART_BATTERY_STATUS(_) => {}
            MavMessage::GENERATOR_STATUS(_) => {}
            MavMessage::ACTUATOR_OUTPUT_STATUS(_) => {}
            MavMessage::TIME_ESTIMATE_TO_TARGET(_) => {}
            MavMessage::TUNNEL(_) => {}
            MavMessage::ONBOARD_COMPUTER_STATUS(_) => {}
            MavMessage::COMPONENT_INFORMATION(_) => {}
            MavMessage::PLAY_TUNE_V2(_) => {}
            MavMessage::SUPPORTED_TUNES(_) => {}
            MavMessage::WHEEL_DISTANCE(_) => {}
            MavMessage::OPEN_DRONE_ID_BASIC_ID(_) => {}
            MavMessage::OPEN_DRONE_ID_LOCATION(_) => {}
            MavMessage::OPEN_DRONE_ID_AUTHENTICATION(_) => {}
            MavMessage::OPEN_DRONE_ID_SELF_ID(_) => {}
            MavMessage::OPEN_DRONE_ID_SYSTEM(_) => {}
            MavMessage::OPEN_DRONE_ID_OPERATOR_ID(_) => {}
            MavMessage::OPEN_DRONE_ID_MESSAGE_PACK(_) => {}
        }
    }
}
