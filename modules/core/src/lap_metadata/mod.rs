use chrono::Utc;

use pipeline::output::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LapMetadata {
    pub identifier: String,
    pub recorded_date: String,
    pub track_id: i8,
    pub era: u8,
    pub tyre_compound: u8,
    pub session_type: u8,
    pub lap_number: u8,
    pub lap_time: f32,
    pub sector_times: [f32; 3],
    pub note: String,
}

impl LapMetadata {
    pub fn new(session: &SessionIdentifier, lap: &Lap, tyre_compound: u8) -> LapMetadata {
        assert!(lap.is_finished);
        assert!(lap.lap_time > 0 as f32);
        assert!(
            (lap.sector_times[0] > 0 as f32)
                & (lap.sector_times[1] > 0 as f32)
                & (lap.sector_times[2] > 0 as f32)
        );

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
            track_id: session.track_id,
            era: session.era,
            tyre_compound: tyre_compound,
            session_type: session.session_type,
            lap_number: lap.lap_number,
            lap_time: lap.lap_time,
            sector_times: lap.sector_times,
            note: String::new(),
        }
    }
}
