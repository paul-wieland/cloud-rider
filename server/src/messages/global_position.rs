use mavlink::common::{BATTERY_STATUS_DATA, GLOBAL_POSITION_INT_DATA};
use serde::Serialize;

#[derive(Serialize)]
pub struct GlobalPosition {
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
    pub relative_alt: f64,
    pub vx: i16,
    pub vy: i16,
    pub vz: i16,
}

#[derive(Serialize)]
pub struct BatteryStatus {
    pub remaining_percent: i8,
    pub temperature_c: f32,
    pub current_battery_ma: i16,
}

#[derive(Serialize)]
pub struct Message<T: Serialize> {
    pub r#type: String,
    pub data: T,
}

// Factory function for GlobalPosition
pub fn from_global_position_int(data: GLOBAL_POSITION_INT_DATA) -> Message<GlobalPosition> {
    let position = GlobalPosition {
        lat: data.lat as f64 / 1e7,
        lon: data.lon as f64 / 1e7,
        alt: data.alt as f64 / 1000.0,
        relative_alt: data.relative_alt as f64 / 1000.0,
        vx: data.vx,
        vy: data.vy,
        vz: data.vz,
    };

    Message {
        r#type: "telemetry".to_string(),
        data: position,
    }
}

// Factory function for BatteryStatus
pub fn from_battery_status(data: BATTERY_STATUS_DATA) -> Message<BatteryStatus> {
    let status = BatteryStatus {
        remaining_percent: data.battery_remaining,
        temperature_c: data.temperature as f32 / 100.0,
        current_battery_ma: data.current_battery,
    };

    Message {
        r#type: "battery".to_string(),
        data: status,
    }
}
