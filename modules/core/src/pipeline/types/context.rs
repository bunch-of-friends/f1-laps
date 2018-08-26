use pipeline::types::*;

#[derive(Debug)]
pub struct Context {
    pub session_context: SessionContext,
    pub history_context: HistoryContext,
}

#[derive(Debug)]
pub struct SessionContext {
    pub header: Header,
    pub session: SessionInfo,
    pub lap: Lap,
    pub sector: Sector,
    pub car_motion: CarMotion,
    pub car_status: CarStatus,
}

#[derive(Debug)]
pub struct HistoryContext {}
