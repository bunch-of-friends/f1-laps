use aggregation::tick::{Lap, Session};
use chrono::Utc;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct LapMetadata {
    pub identifier: String,
    pub recorded_date: String,
    pub track_id: u8,
    pub team_id: u8,
    pub era: i16, //TODO this should be u16 - left for now to keep debug data compatible
    pub tyre_compound: u8,
    pub session_type: u8,
    pub lap_number: u8,
    pub lap_time: f32,
    pub sector_times: [f32; 3],
    pub note: String,
}

impl LapMetadata {
    pub fn new(lap: &Lap, session: &Session) -> LapMetadata {
        let date = Utc::now();
        let identifier = format!(
            "lap_{}_track-{:02}_L{:03}.bin",
            date.format("%Y-%m-%d-%H-%M-%S-%f"),
            session.track_id,
            lap.lap_number
        );

        LapMetadata {
            identifier: identifier,
            recorded_date: date.to_rfc3339(),
            track_id: session.track_id as u8,
            team_id: session.team_id as u8,
            era: session.era as i16,
            tyre_compound: lap.tyre_compound as u8,
            session_type: session.session_type as u8,
            lap_number: lap.lap_number as u8,
            lap_time: lap.lap_time,
            sector_times: [lap.sector1_time, lap.sector2_time, lap.sector3_time],
            note: String::new(),
        }
    }
}
