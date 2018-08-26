use pipeline::types::Tick;

#[derive(Debug)]
pub struct Output {
    pub tick: Tick,
    pub labels: Labels,
    pub events: Events,
}

#[derive(Debug)]
pub struct Labels {
    pub is_new_session: bool,
    pub is_new_lap: bool,
    pub is_new_sector: bool,
    pub is_flashback: bool,
    pub is_teleported: bool,
    pub current_lap: Lap,
    pub current_sector: Sector,
    pub tyre_compound: u8,
}

#[derive(Debug)]
pub struct Events {
    pub started_session: Option<Session>,
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

#[derive(Debug)]
pub struct Lap {
    pub lap_number: u8,
    pub sector_times: [f32; 3],
    pub lap_time: f32,
    pub is_finished: bool,
}

#[derive(Debug)]
pub struct Sector {
    pub sector_number: u8,
    pub sector_time: f32,
    pub is_finished: bool,
}

