use aggregation::tick::{Lap, LiveData, Sector, Session, Tick};

pub struct Collector {
    session: Option<Session>,
    live_data: Option<LiveData>,
    lap: Option<Lap>,
    sector: Option<Sector>,
}

impl Collector {
    pub fn new() -> Collector {
        Collector {
            session: None,
            live_data: None,
            lap: None,
            sector: None,
        }
    }

    pub fn set_data(&mut self, tick: Tick) {
        self.live_data = Some(tick.live_data);

        if let Some(session) = tick.session_started {
            self.session = Some(session);
        }

        if let Some(lap) = tick.lap_finished {
            self.lap = Some(lap);
        }

        if let Some(sector) = tick.sector_finished {
            self.sector = Some(sector);
        }
    }

    pub fn get_data(
        &mut self,
    ) -> (
        Option<LiveData>,
        Option<Session>,
        Option<Lap>,
        Option<Sector>,
    ) {
        let res = (self.live_data, self.session, self.lap, self.sector);

        self.live_data = None;
        self.session = None;
        self.lap = None;
        self.sector = None;

        return res;
    }
}
