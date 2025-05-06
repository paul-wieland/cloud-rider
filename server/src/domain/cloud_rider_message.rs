use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum CloudRiderMessage {
    Heartbeat(Heartbeat),
    GlobalPosition(GlobalPosition),
    BatteryStatus(BatteryStatus),
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalPosition {
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
    pub relative_alt: f64,
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
    pub temperature_c: f32,
    pub current_battery_ma: i16,
}
