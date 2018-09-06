use chrono::serde::ts_nanoseconds;
use chrono::{DateTime, Utc};

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
pub struct LapData {
    
}