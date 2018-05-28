#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate chrono;

pub mod aggregation;
pub mod storage;
pub mod udp;

use aggregation::tick::{Lap, LiveData, Sector, Session, Tick};

use std::sync::mpsc;
use std::thread;

static mut DATA_HOLDER: DataHolder = DataHolder {
    session: None,
    live_data: None,
    lap: None,
    sector: None,
};

pub fn start_listening(port: i32, should_store_replay: bool) {
    storage::ensure_storage_files_created();
    aggregation::preload_records();

    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    thread::spawn(move || {
        udp::start_listening(port, tx, should_store_replay);
    });

    thread::spawn(move || loop {
        receive_tick(&rx);
    });
}

pub fn get_next_tick() -> Option<Tick> {
    if unsafe { !DATA_HOLDER.has_data() } {
        return None;
    }

    let data = unsafe { DATA_HOLDER.get_data() };

    let tick = Tick {
        live_data: data.0,
        session_started: data.1,
        lap_finished: data.2,
        sector_finished: data.3,
    };
    return Some(tick);
}

pub fn replay_data() {
    storage::ensure_storage_files_created();
    aggregation::preload_records();

    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    thread::spawn(move || {
        storage::replay::replay_laps(tx);
    });

    thread::spawn(move || loop {
        receive_tick(&rx);
    });
}

fn receive_tick(rx: &mpsc::Receiver<Tick>) {
    let tick_result = rx.recv().ok();

    if let Some(tick) = tick_result {
        unsafe {
            DATA_HOLDER.set_data(tick);
        }
    }
}

struct DataHolder {
    session: Option<Session>,
    live_data: Option<LiveData>,
    lap: Option<Lap>,
    sector: Option<Sector>,
}

impl DataHolder {
    pub fn has_data(&self) -> bool {
        return self.live_data.is_some();
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

    pub fn get_data(&mut self) -> (LiveData, Option<Session>, Option<Lap>, Option<Sector>) {
        let res = (
            self.live_data.expect("live data not set"),
            self.session,
            self.lap,
            self.sector,
        );

        self.live_data = None;
        self.session = None;
        self.lap = None;
        self.sector = None;

        return res;
    }
}
