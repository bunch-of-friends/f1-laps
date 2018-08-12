pub(crate) trait DataReceiver {
    fn start_receiving();
    fn receive(input_tick: InputTick);
}

pub(crate) trait DataProcessor {
    fn build_labels(&self, input_tick: &InputTick, context: &Context) -> PacketLabels;
    fn build_stats(&self, input_tick: &InputTick, context: &Context, labels: &PacketLabels) -> PacketStats;
    fn build_context(&self, input_tick: &InputTick, context: &Context, labels: &PacketLabels) -> Context;
}

pub(crate) struct InputTick {
    pub session_time: f32,
    pub session_distance: f32,
    pub lap_time: f32,
    pub lap_distance: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub speed: f32,
    pub throttle: f32,
    pub steer: f32,
    pub brake: f32,
    pub gear: f32,
    pub lap_number: u8,
    pub engine_rate: f32,
    pub car_position: u8,
    pub drs: bool,
    pub sector: u8,
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
    pub current_lap_invalid: bool,
    pub is_spectating: bool,
    pub cars_total: u8,
}

pub(crate) struct PacketLabels {
    pub is_new_session: bool,
    pub is_new_lap: bool,
    pub is_new_sector: bool,
}

pub(crate) struct PacketStats {
    pub previous_lap: Option<FinishedLap>,
    pub previous_sector: Option<FinishedSector>,
    pub session: Session,
}

pub(crate) struct FinishedLap {
    number: u8,
    sectors: [f32; 3],
    tyre_compound: u8,
}

pub(crate) struct FinishedSector {
    number: u8,
    time: f32,
    tyre_compound: u8,
}

pub(crate) struct Session {
    pub track_id: u8,
    pub session_type: u8,
    pub team_id: u8,
    pub era: u16,
}

pub(crate) struct Context {
    pub session_context: SessionContext,
    pub history_context: HistoryContext,
}

pub(crate) struct SessionContext {}

pub(crate) struct HistoryContext {}
