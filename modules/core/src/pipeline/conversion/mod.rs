use pipeline::types::*;

impl Tick {
    pub fn new(header: Header) -> Tick {
        Tick {
            header: header,
            session_info: None,
            lap_data: None,
            car_motion: None,
            car_telemetry: None,
            car_status: None,
        }
    }
}

impl Session {
    pub fn from_tick(tick: &Tick) -> Option<Session> {
        if let Some(ref s) = tick.session_info {
            Some(Session {
                track_id: s.track_id,
                session_type: s.session_type,
                era: s.era,
                uid: tick.header.session_uid,
            })
        } else {
            None
        }
    }

    pub fn empty() -> Session {
        Session {
            track_id: 0,
            session_type: 0,
            era: 0,
            uid: 0,
        }
    }
}

impl Lap {
    pub fn from_tick(tick: &Tick) -> Option<Lap> {
        if let Some(ref lap_data) = tick.lap_data {
            Some(Lap {
                lap_number: lap_data.current_lap_number,
                sector_times: [lap_data.sector1_time, lap_data.sector2_time, 0 as f32],
                lap_time: lap_data.current_lap_time,
                is_finished: false,
            })
        } else {
            None
        }
    }

    pub fn empty() -> Lap {
        Lap {
            lap_number: 0,
            sector_times: [0 as f32; 3],
            lap_time: 0 as f32,
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
    pub fn from_tick(tick: &Tick) -> Option<Sector> {
        if let Some(ref lap_data) = tick.lap_data {
            Some(Sector {
                sector_number: lap_data.current_sector_number,
                sector_time: 0 as f32,
                is_finished: false,
            })
        } else {
            None
        }
    }

    pub fn empty() -> Sector {
        Sector {
            sector_number: 0,
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

impl CarStatus {
    pub fn empty() -> CarStatus {
        CarStatus { tyre_compound: 0 }
    }
}

impl SessionContext {
    pub fn empty() -> SessionContext {
        SessionContext {
            session: Session::empty(),
            lap: Lap::empty(),
            sector: Sector::empty(),
            car_position: CarPosition::empty(),
            car_status: CarStatus::empty(),
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
