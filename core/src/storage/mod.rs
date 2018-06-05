pub mod lap;
pub mod lap_store;
pub mod record;
pub mod record_store;

use bincode;
use std::collections::HashMap;
use std::fs::{create_dir, File};
use std::path::Path;

use self::lap::LapMetadata;
use self::lap_store::LapStore;
use self::record::RecordSet;
use self::record_store::RecordStore;
use udp::packet::Packet;

static mut RECORD_STORE: RecordStore = RecordStore { record_sets: None };
static mut LAP_STORE: LapStore = LapStore {
    laps_metadata: None,
};

pub fn initialise() {
    ensure_storage_files_created();
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

fn persist_record_store(record_store: &RecordStore) {
    let file = File::create("storage/records.bin").expect("failed to create records file");
    bincode::serialize_into(file, &record_store.record_sets)
        .expect("failed to serialise records file");
}

fn ensure_storage_files_created() {
    let folder_path = "storage/laps";
    if !Path::new(folder_path).exists() {
        match create_dir(folder_path) {
            Ok(_) => (),
            Err(e) => panic!("failed to create folder, path: {},  e: {}", folder_path, e),
        }
    }

    let records_path = "storage/records.bin";
    if !Path::new(records_path).exists() {
        match File::create(records_path) {
            Ok(_) => (),
            Err(e) => panic!("failed to create records file: {}", e),
        }
    }

    let records_path = "storage/laps.bin";
    if !Path::new(records_path).exists() {
        match File::create(records_path) {
            Ok(_) => (),
            Err(e) => panic!("failed to create laps file: {}", e),
        }
    }
}

fn load_stores() {
    let records_store = load_record_store();
    unsafe {
        RECORD_STORE = records_store;
    }

    let lap_store = load_lap_store();
    unsafe {
        LAP_STORE = lap_store;
    }
}

fn load_record_store() -> RecordStore {
    let file = File::open("storage/records.bin").expect("failed to open records file");
    match bincode::deserialize_from::<File, HashMap<String, RecordSet>>(file) {
        Ok(x) => RecordStore::new(x),
        Err(_) => RecordStore::new(HashMap::<String, RecordSet>::new()),
    }
}

fn load_lap_store() -> LapStore {
    let file = File::open("storage/laps.bin").expect("failed to open records file");
    match bincode::deserialize_from::<File, Vec<LapMetadata>>(file) {
        Ok(x) => LapStore::new(x),
        Err(e) => {
            println!("error opening laps file: {}", e);
            LapStore::new(Vec::new())
        }
    }
}
