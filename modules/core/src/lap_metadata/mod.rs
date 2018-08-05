use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LapMetadata {
    pub identifier: String,
    pub recorded_date: String,
    pub track_id: u8,
    pub team_id: u8,
    pub era: u16,
    pub tyre_compound: u8,
    pub session_type: u8,
    pub lap_number: u8,
    pub lap_time: f32,
    pub sector_times: [f32; 3],
    pub is_valid: bool,
    pub note: String,
}

impl LapMetadata {
    pub fn new(
        session_type: u8,
        track_id: u8,
        team_id: u8,
        era: u16,
        tyre_compound: u8,
        lap_number: u8,
        lap_time: [f32; 4],
        is_valid: bool,
    ) -> LapMetadata {
        let date = Utc::now();
        let identifier = format!(
            "lap_{}_track-{:02}_L{:03}.bin",
            date.format("%Y-%m-%d-%H-%M-%S-%f"),
            track_id,
            lap_number
        );

        LapMetadata {
            identifier: identifier,
            recorded_date: date.to_rfc3339(),
            track_id: track_id,
            team_id: team_id,
            era: era,
            tyre_compound: tyre_compound,
            session_type: session_type,
            lap_number: lap_number,
            lap_time: lap_time[0],
            sector_times: [lap_time[1], lap_time[2], lap_time[3]],
            is_valid: is_valid,
            note: String::new(),
        }
    }
}
