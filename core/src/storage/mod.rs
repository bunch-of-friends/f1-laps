mod file_system;
pub mod lap;
mod lap_store;
pub mod path_helper;
pub mod record;

use self::lap::LapMetadata;
use self::lap_store::LapStore;
use udp::packet::Packet;

static mut LAP_STORE: LapStore = LapStore {
    laps_metadata: None,
    path_helper: None,
};

pub fn initialise(storage_folder_path: &str) {
    let store = file_system::initialise(storage_folder_path);
    unsafe {
        LAP_STORE = store;
    }
}

pub fn get_all_laps_metadata() -> Vec<LapMetadata> {
    unsafe {
        return LAP_STORE.get_all_laps_metadata();
    }
}

pub fn get_all_laps_data() -> Vec<Packet> {
    unsafe {
        return LAP_STORE.get_all_laps_data();
    }
}

pub fn get_lap_data(identifier: &str) -> Option<Vec<Packet>> {
    unsafe {
        return LAP_STORE.get_lap_data(&identifier);
    }
}

pub fn store_lap(packets: Vec<Packet>, metadata: LapMetadata) {
    unsafe {
        LAP_STORE.store_lap(packets, metadata);
    }
}
