pub struct InputTick {
    pub session_time: f32,
    pub session_distance: f32,
    pub lap_number: u8,
    pub lap_time: f32,
    pub lap_distance: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub speed: f32,
    pub throttle: f32,
    pub steer: f32,
    pub brake: f32,
    pub gear: u8,
    pub engine_rate: f32,
    pub car_position: u8,
    pub is_drs_open: bool,
    pub sector_number: u8,
    pub sector1_time: f32,
    pub sector2_time: f32,
    pub team_id: u8,
    pub total_laps: u8,
    pub last_lap_time: f32,
    pub max_gears: u8,
    pub session_type: u8,
    pub track_id: u8,
    pub vehicle_fia_flags: i8,
    pub era: u16,
    pub tyre_compound: u8,
    pub is_current_lap_valid: bool,
    pub is_spectating: bool,
    pub cars_total: u8,
}

pub struct PacketLabels {
    pub is_new_session: bool,
    pub is_new_lap: bool,
    pub is_new_sector: bool,
    pub current_session: Session,
    pub current_lap: Lap,
    pub current_sector: Sector,
}

pub struct PacketStats {
    pub finished_sector: Option<Sector>,
    pub finished_lap: Option<Lap>,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub track_id: u8,
    pub session_type: u8,
    pub team_id: u8,
    pub era: u16,
}

pub struct Lap {
    pub lap_number: u8,
    pub sector_times: [f32; 3],
    pub lap_time: f32,
}

pub struct Sector {
    pub sector_number: u8,
    pub sector_time: f32,
}

pub struct Context {
    pub session_context: SessionContext,
    pub history_context: HistoryContext,
}

pub struct SessionContext {
    pub session: Session,
    pub lap: Lap,
    pub sector: Sector,
}

pub struct HistoryContext {}

pub struct StoreLapResult {}

pub struct StoreMetadataResult {}

pub struct PipelineResult {
    pub labels: PacketLabels,
    pub stats: PacketStats,
    pub lap_store_result: StoreLapResult,
    pub metadata_store_result: StoreMetadataResult,
    pub new_context: Context,
}
