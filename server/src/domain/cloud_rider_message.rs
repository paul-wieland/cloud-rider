use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum CloudRiderMessage {
    Heartbeat(Heartbeat),
    GlobalPosition(GlobalPosition),
    BatteryStatus(BatteryStatus),
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalPosition {
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub relative_alt: i32,
    pub vx: i16,
    pub vy: i16,
    pub vz: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    pub timestamp: String,
}

impl Heartbeat {
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryStatus {
    pub remaining_percent: i8,
    pub temperature_c: i16,
    pub current_battery_ma: i16,
}
