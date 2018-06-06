mod file_system;
pub mod lap;
mod lap_store;
pub mod record;
mod record_store;

use self::lap::LapMetadata;
use self::lap_store::LapStore;
use self::record_store::RecordStore;
use udp::packet::Packet;

static mut RECORD_STORE: RecordStore = RecordStore { record_sets: None };
static mut LAP_STORE: LapStore = LapStore {
    laps_metadata: None,
};

pub fn initialise() {
    file_system::ensure_storage_files_created();
    load_stores();
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

fn load_stores() {
    let records_store = file_system::load_record_store();
    unsafe {
        RECORD_STORE = records_store;
    }

    let lap_store = file_system::load_lap_store();
    unsafe {
        LAP_STORE = lap_store;
    }
}
