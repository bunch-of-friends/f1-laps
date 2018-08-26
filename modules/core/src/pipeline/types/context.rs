use pipeline::types::{Lap, Sector, Session};

#[derive(Debug)]
pub struct Context {
    pub session_context: SessionContext,
    pub history_context: HistoryContext,
}

#[derive(Debug)]
pub struct SessionContext {
    pub session: Session,
    pub lap: Lap,
    pub sector: Sector,
    pub position: Position,
}

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct HistoryContext {}
