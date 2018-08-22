pub mod data_store;

use self::data_store::DataStore;
use file_system;
use file_system::path_helper;
use lap_metadata::LapMetadata;
use pipeline::types::Tick;
use record_tracking::record_tracker::RecordTracker;
use record_tracking::RecordSet;
use std::sync::Mutex;
use udp::packet::Packet;

lazy_static! {
    static ref DATA_STORE: Mutex<DataStore> = Mutex::new(DataStore::new());
}

pub fn initialise(storage_folder_path: &str) {
    let fs_init_result = file_system::initialise(storage_folder_path);
    DATA_STORE
        .lock()
        .unwrap()
        .initialise(fs_init_result.0, fs_init_result.1, fs_init_result.2)
}

pub fn get_all_laps_metadata() -> Vec<LapMetadata> {
    DATA_STORE.lock().unwrap().get_all_laps_metadata()
}

pub fn get_all_records() -> RecordSet {
    DATA_STORE.lock().unwrap().get_all_records()
}

pub fn get_all_packets() -> Vec<Packet> {
    DATA_STORE.lock().unwrap().get_all_packets()
}

pub fn get_lap_data(identifier: &str) -> Option<Vec<Tick>> {
    DATA_STORE.lock().unwrap().get_lap_data(&identifier)
}

pub fn store_lap(ticks: Vec<Tick>, metadata: &LapMetadata) {
    DATA_STORE.lock().unwrap().store_lap(ticks, metadata)
}

pub fn get_record_tracker(track_id: u8, era: u16) -> RecordTracker {
    DATA_STORE.lock().unwrap().get_record_tracker(track_id, era)
}

pub fn store_records(record_tracker: &RecordTracker) {
    DATA_STORE.lock().unwrap().store_records(record_tracker)
}
