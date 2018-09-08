use chrono::serde::ts_nanoseconds;
use chrono::{DateTime, Utc};
use pipeline::output::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LapHeader {
    pub id: String,
    #[serde(with = "ts_nanoseconds")]
    pub recorded_date: DateTime<Utc>,
    pub track_id: i8,
    pub team_id: u8,
    pub era: u8,
    pub tyre_compound: u8,
    pub weather: u8,
    pub session_type: u8,
    pub lap_number: u8,
    pub lap_time: f32,
    pub sector_times: [f32; 3],
    pub note: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LapTelemetry {
    pub id: String,
    pub session_data: SessionData,
    pub lap_data: Vec<LapData>,
    pub car_status: Vec<CarStatus>,
    pub car_telemetry: Vec<CarTelemetry>,
    pub car_motion: Vec<CarMotion>,
    pub car_setup: CarSetup,
    pub participants_info: ParticipantInfo,
}
