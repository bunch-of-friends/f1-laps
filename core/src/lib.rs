#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate chrono;

pub mod models;
mod storage;
mod aggregation;
mod udp;

use models::api::{BestLap, BestSector, LiveData, Session, Tick};
use std::{thread, time};
use std::sync::mpsc;

static mut SESSION: Option<Session> = None;
static mut LIVE_DATA: Option<LiveData> = None;
static mut BEST_SESSION_LAP: Option<BestLap> = None;
static mut BEST_SESSION_SECTOR: Option<BestSector> = None;
static mut BEST_EVER_LAP: Option<BestLap> = None;
static mut BEST_EVER_SECTOR: Option<BestSector> = None;

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

pub fn get_next_tick() -> Tick {
    let tick = unsafe {
        Tick {
            session: SESSION,
            live_data: LIVE_DATA,
            best_ever_lap: BEST_SESSION_LAP,
            best_ever_sector: BEST_SESSION_SECTOR,
            best_session_lap: BEST_EVER_LAP,
            best_session_sector: BEST_EVER_SECTOR,
        }
    };

    unsafe {
        SESSION = None;
        LIVE_DATA = None;
        BEST_SESSION_LAP = None;
        BEST_SESSION_SECTOR = None;
        BEST_EVER_LAP = None;
        BEST_EVER_SECTOR = None
    }

    tick
}

pub fn replay_data(frequency_hz: u64) {
    storage::ensure_storage_files_created();
    aggregation::preload_records();

    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    thread::spawn(move || {
        storage::replay::get_replay_data(tx);
    });

    let tick_delay = time::Duration::from_millis(1000 / frequency_hz);
    thread::spawn(move || loop {
        receive_tick(&rx);
        thread::sleep(tick_delay);
    });
}

fn receive_tick(rx: &mpsc::Receiver<Tick>) {
    let tick_result = rx.recv().ok();
    if tick_result.is_none() {
        return;
    }

    let tick = tick_result.unwrap();

    if tick.live_data.is_some() {
        unsafe { LIVE_DATA = tick.live_data }
    }

    if tick.session.is_some() {
        unsafe { SESSION = tick.session }
    }

    if tick.best_session_lap.is_some() {
        unsafe { BEST_SESSION_LAP = tick.best_session_lap }
    }

    if tick.best_session_sector.is_some() {
        unsafe { BEST_SESSION_SECTOR = tick.best_session_sector }
    }

    if tick.best_ever_lap.is_some() {
        unsafe { BEST_EVER_LAP = tick.best_ever_lap }
    }

    if tick.best_ever_sector.is_some() {
        unsafe { BEST_EVER_SECTOR = tick.best_ever_sector }
    }
}
