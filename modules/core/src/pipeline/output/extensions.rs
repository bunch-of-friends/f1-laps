use pipeline::input::*;
use pipeline::output::*;

impl SessionIdentifier {
    pub fn from(s: &SessionData, header: &Header) -> SessionIdentifier {
        SessionIdentifier {
            track_id: s.track_id,
            session_type: s.session_type,
            era: s.era,
            session_uid: header.session_uid,
            session_time: header.session_time,
        }
    }
}

impl Lap {
    pub fn from(lap_data: &LapData) -> Lap {
        Lap {
            lap_number: lap_data.current_lap_number,
            sector_times: [lap_data.sector1_time, lap_data.sector2_time, 0 as f32],
            lap_time: lap_data.current_lap_time,
            is_complete: false,
        }
    }

    pub fn completed(s1_t: f32, s2_t: f32, s3_t: f32, lap_t: f32, lap_n: u8) -> Lap {
        assert!(lap_n > 0);
        assert!(((s1_t + s2_t + s3_t) - lap_t).abs() < 0.001);

        Lap {
            lap_number: lap_n,
            sector_times: [s1_t, s2_t, s3_t],
            lap_time: lap_t,
            is_complete: true,
        }
    }
}

impl Sector {
    pub fn from(lap_data: &LapData) -> Sector {
        Sector {
            sector_number: lap_data.current_sector_number,
            sector_time: 0 as f32,
            is_complete: false,
        }
    }

    pub fn completed(t: f32, n: u8) -> Sector {
        Sector {
            sector_number: n,
            sector_time: t,
            is_complete: true,
        }
    }
}

impl SessionContext {
    pub fn empty() -> SessionContext {
        SessionContext {
            header: None,
            current_session: None,
            current_lap: None,
            current_sector: None,
            car_motion: None,
            car_status: None,
            car_setup: None,
            participants_info: None,
            session_data: None,
        }
    }
}

impl Context {
    pub fn empty() -> Context {
        Context {
            session_context: SessionContext::empty(),
            history_context: HistoryContext {},
        }
    }
}
