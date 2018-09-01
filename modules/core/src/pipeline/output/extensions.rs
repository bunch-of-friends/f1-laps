use pipeline::input::*;
use pipeline::output::*;

impl SessionIdentifier {
    pub fn from_session_data(s: &SessionData, header: &Header) -> SessionIdentifier {
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
    pub fn from_tick(tick: &Tick) -> Lap {
        Lap {
            lap_number: tick.lap_data.current_lap_number,
            sector_times: [
                tick.lap_data.sector1_time,
                tick.lap_data.sector2_time,
                0 as f32,
            ],
            lap_time: tick.lap_data.current_lap_time,
            is_finished: false,
        }
    }

    pub fn finished(s1_t: f32, s2_t: f32, s3_t: f32, lap_t: f32, lap_n: u8) -> Lap {
        assert!(lap_n > 0);
        assert!((s1_t + s2_t + s3_t) == lap_t);

        Lap {
            lap_number: lap_n,
            sector_times: [s1_t, s2_t, s3_t],
            lap_time: lap_t,
            is_finished: true,
        }
    }
}

impl Sector {
    pub fn from_tick(tick: &Tick) -> Sector {
        Sector {
            sector_number: tick.lap_data.current_sector_number,
            sector_time: 0 as f32,
            is_finished: false,
        }
    }

    pub fn finished(t: f32, n: u8) -> Sector {
        Sector {
            sector_number: n,
            sector_time: t,
            is_finished: true,
        }
    }
}

impl SessionContext {
    pub fn empty() -> SessionContext {
        SessionContext {
            header: None,
            session: None,
            lap: None,
            sector: None,
            car_motion: None,
            car_status: None,
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
