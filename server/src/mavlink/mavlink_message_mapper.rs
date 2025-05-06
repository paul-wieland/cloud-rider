use crate::domain::cloud_rider_message::{
    BatteryStatus, CloudRiderMessage, GlobalPosition, Heartbeat,
};
use chrono::Utc;
use mavlink::common::MavAutopilot::MAV_AUTOPILOT_INVALID;
use mavlink::common::MavState::MAV_STATE_ACTIVE;
use mavlink::common::MavType::MAV_TYPE_GCS;
use mavlink::common::{MavMessage, MavModeFlag, HEARTBEAT_DATA};

pub fn to_cloud_rider(message: &MavMessage) -> Option<CloudRiderMessage> {
    match message {
        MavMessage::HEARTBEAT(_) => Some(CloudRiderMessage::Heartbeat(Heartbeat {
            timestamp: Utc::now().to_rfc3339(),
        })),
        MavMessage::GLOBAL_POSITION_INT(mavlink_data) => {
            Some(CloudRiderMessage::GlobalPosition(GlobalPosition {
                lat: mavlink_data.lat,
                lon: mavlink_data.lon,
                alt: mavlink_data.alt,
                relative_alt: mavlink_data.relative_alt,
                vx: mavlink_data.vx,
                vy: mavlink_data.vy,
                vz: mavlink_data.vz,
            }))
        }
        MavMessage::BATTERY_STATUS(mav_battery_status) => {
            Some(CloudRiderMessage::BatteryStatus(BatteryStatus {
                remaining_percent: mav_battery_status.battery_remaining,
                temperature_c: mav_battery_status.temperature,
                current_battery_ma: mav_battery_status.current_battery,
            }))
        }
        _ => None,
    }
}

pub fn to_mavlink(message: &CloudRiderMessage) -> Option<MavMessage> {
    match message {
        CloudRiderMessage::Heartbeat(_) => Some(MavMessage::HEARTBEAT(HEARTBEAT_DATA {
            custom_mode: 0,
            mavtype: MAV_TYPE_GCS,
            autopilot: MAV_AUTOPILOT_INVALID,
            base_mode: MavModeFlag::MAV_MODE_FLAG_CUSTOM_MODE_ENABLED,
            system_status: MAV_STATE_ACTIVE,
            mavlink_version: 3,
        })),
        CloudRiderMessage::GlobalPosition(_) => None,
        CloudRiderMessage::BatteryStatus(_) => None,
        CloudRiderMessage::Error(_) => None,
    }
}
