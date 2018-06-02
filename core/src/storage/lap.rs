#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct LapMetadata {
    identifier: String,
    recorded_date: String,
    track_id: f32,
    team_id: f32,
    tyre_compount: f32,
    session_type: f32,
    lap_number: f32,
    lap_time: f32,
    sector_times: [f32; 3],
    note: String,
}
