#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate chrono;

pub mod aggregation;
pub mod record_tracking;
pub mod replay;
pub mod storage;
pub mod udp;
pub mod lap_metadata;

use aggregation::tick::{Lap, LiveData, Sector, Session, Tick};
use lap_metadata::LapMetadata;
use std::sync::mpsc;
use std::thread;

static mut DATA_HOLDER: DataHolder = DataHolder {
    session: None,
    live_data: None,
    lap: None,
    sector: None,
};

pub fn initialise(storage_folder_path: String) {
    storage::initialise(&storage_folder_path);
}

pub fn start_listening(port: i32, should_store_replay: bool) {
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

pub fn get_all_laps_metadata() -> Vec<LapMetadata> {
    return storage::get_all_laps_metadata();
}

pub fn get_lap_data(identifier: String) -> Vec<LiveData> {
    let packets = match storage::get_lap_data(&identifier) {
        Some(x) => x,
        None => panic!("no lap data found for identifier: {}", identifier), // TODO: add some sort of messaging/feedback mechanism
    };

    return aggregation::convert_packets(&packets);
}

pub fn replay_lap(identifier: String) {
    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    thread::spawn(move || match storage::get_lap_data(&identifier) {
        Some(packets) => replay::stream_packets(tx, &packets, false, false),
        None => println!("no lap data found for identifier: {}", identifier), // TODO: add some sort of messaging/feedback mechanism
    });

    thread::spawn(move || loop {
        receive_tick(&rx);
    });
}

pub fn replay_all_laps() {
    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    // always false for replayed data, but can be set to true for debugging/testing purposes
    // will will store all the packets (duplicating them) like if they were received via udp
    let should_store_packets = false;

    // only set to true for debugging, all packets will be streamed at once
    let disable_sleep = false;

    thread::spawn(move || {
        let packets = storage::get_all_laps_data();
        replay::stream_packets(tx, &packets, should_store_packets, disable_sleep);
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
