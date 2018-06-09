mod data_store;
mod file_system;
mod path_helper;

use self::data_store::DataStore;
use lap_metadata::LapMetadata;
use udp::packet::Packet;
use record_tracking::record_tracker::RecordTracker;

static mut DATA_STORE: DataStore = DataStore {
    laps_metadata: None,
    record_set: None,
    path_helper: None,
};

pub fn initialise(storage_folder_path: &str) {
    let store = file_system::initialise(storage_folder_path);
    unsafe {
        DATA_STORE = store;
    }
}

pub fn get_all_laps_metadata() -> Vec<LapMetadata> {
    unsafe {
        return DATA_STORE.get_all_laps_metadata();
    }
}

pub fn get_all_laps_data() -> Vec<Packet> {
    unsafe {
        return DATA_STORE.get_all_laps_data();
    }
}

pub fn get_lap_data(identifier: &str) -> Option<Vec<Packet>> {
    unsafe {
        return DATA_STORE.get_lap_data(&identifier);
    }
}

pub fn store_lap(packets: Vec<Packet>, metadata: LapMetadata) {
    unsafe {
        DATA_STORE.store_lap(packets, metadata);
    }
}

    pub fn get_record_tracker(track_id: u8, era: u16) -> RecordTracker {
        unsafe {
            return DATA_STORE.get_record_tracker(track_id, era);
        }
    }
