#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate bincode;
extern crate chrono;

pub mod aggregation;
pub mod file_system;
pub mod lap_metadata;
pub mod record_tracking;
pub mod replay;
pub mod storage;
pub mod udp;

use aggregation::collector::Collector;
use aggregation::tick::{LiveData, Tick};
use lap_metadata::LapMetadata;
use record_tracking::RecordSet;
use std::sync::{mpsc, Mutex};
use std::thread;

lazy_static! {
    static ref DATA_HOLDER: Mutex<Collector> = Mutex::new(Collector::new());
}

pub fn initialise(storage_folder_path: String) {
    storage::initialise(&storage_folder_path);
}

pub fn start_listening(port: i32) {
    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    thread::spawn(move || {
        udp::start_listening(port, tx);
    });

    thread::spawn(move || loop {
        receive_tick(&rx);
    });
}

pub fn get_next_tick() -> Option<Tick> {
    let mut data_holder = DATA_HOLDER.lock().unwrap();
    if data_holder.has_data() {
        return None;
    }

    let data = data_holder.get_data();

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

pub fn get_all_records() -> RecordSet {
    return storage::get_all_records();
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
        Some(packets) => replay::stream_packets(tx, packets),
        None => println!("no lap data found for identifier: {}", identifier), // TODO: add some sort of messaging/feedback mechanism
    });

    thread::spawn(move || loop {
        receive_tick(&rx);
    });
}

pub fn replay_all_laps() {
    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    thread::spawn(move || {
        let packets = storage::get_all_laps_data();
        replay::stream_packets(tx, packets);
    });

    thread::spawn(move || loop {
        receive_tick(&rx);
    });
}

fn receive_tick(rx: &mpsc::Receiver<Tick>) {
    let tick_result = rx.recv().ok();

    if let Some(tick) = tick_result {
        DATA_HOLDER.lock().unwrap().set_data(tick);
    }
}
