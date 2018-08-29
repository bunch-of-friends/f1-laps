mod extensions;

pub use pipeline::input::{CarMotion, CarStatus, CarTelemetry, Header, SessionData};

#[derive(Debug)]
pub struct Context {
    pub session_context: SessionContext,
    pub history_context: HistoryContext,
}

#[derive(Debug)]
pub struct SessionContext {
    pub header: Option<Header>,
    pub session: Option<SessionIdentifier>,
    pub lap: Option<Lap>,
    pub sector: Option<Sector>,
    pub car_motion: Option<CarMotion>,
    pub car_status: Option<CarStatus>,
}

#[derive(Debug)]
pub struct HistoryContext {}

#[derive(Debug)]
pub struct Output {
    pub labels: Labels,
    pub events: Events,
    pub session_data: Option<SessionData>,
    pub car_status: Option<CarStatus>,
    pub car_telemetry: Option<CarTelemetry>,
    pub car_motion: Option<CarMotion>,
}

#[derive(Debug)]
pub struct Labels {
    pub is_new_session: bool,
    pub is_new_lap: bool,
    pub is_new_sector: bool,
    pub is_flashback: bool,
    pub is_teleported: bool,
}

#[derive(Debug)]
pub struct Events {
    pub started_session: Option<SessionIdentifier>,
    pub finished_sector: Option<Sector>,
    pub finished_lap: Option<Lap>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionIdentifier {
    pub track_id: i8,
    pub session_type: u8,
    pub era: u8,
    pub session_uid: u64,
    pub session_time: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lap {
    pub lap_number: u8,
    pub sector_times: [f32; 3],
    pub lap_time: f32,
    pub is_finished: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sector {
    pub sector_number: u8,
    pub sector_time: f32,
    pub is_finished: bool,
}
