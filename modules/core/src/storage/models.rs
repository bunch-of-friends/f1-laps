use chrono::serde::ts_nanoseconds;
use chrono::{DateTime, Utc};
use pipeline::output::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LapHeader {
    pub identifier: String,
    #[serde(with = "ts_nanoseconds")]
    pub recorded_date: DateTime<Utc>,
    pub track_id: u8,
    pub team_id: u8,
    pub era: u16,
    pub tyre_compound: u8,
    pub session_type: u8,
    pub lap_number: u8,
    pub lap_time: f32,
    pub sector_times: [f32; 3],
    pub note: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LapTelemetry {
    pub session_data: SessionData,
    pub lap_data: Vec<OptMultiCarData<LapData>>,
    pub car_status: Vec<OptMultiCarData<CarStatus>>,
    pub car_telemetry: Vec<OptMultiCarData<CarTelemetry>>,
    pub car_motion: Vec<OptMultiCarData<CarMotion>>,
    pub car_setup: OptMultiCarData<CarSetup>,
    pub participants_info: OptMultiCarData<ParticipantInfo>,
}
