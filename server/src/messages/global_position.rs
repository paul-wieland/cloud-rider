use mavlink::common::GLOBAL_POSITION_INT_DATA;
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

impl GlobalPosition {
    pub fn from_global_position_int(data: GLOBAL_POSITION_INT_DATA) -> Self {
        Self {
            lat: data.lat as f64 / 1e7,
            lon: data.lon as f64 / 1e7,
            alt: data.alt as f64 / 1000.0,
            relative_alt: data.relative_alt as f64 / 1000.0,
            vx: data.vx,
            vy: data.vy,
            vz: data.vz,
        }
    }
}
