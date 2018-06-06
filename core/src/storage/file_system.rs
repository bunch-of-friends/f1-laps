use bincode;
use std::collections::HashMap;
use std::fs::{create_dir, read_dir, File};
use std::path::Path;
use storage::lap::LapMetadata;
use storage::lap_store::LapStore;
use storage::record::RecordSet;
use storage::record_store::RecordStore;
use udp::packet::Packet;

static mut ROOT_DIR: &str = ".";
const STORAGE_DIR: &'static str = "storage";
const LAPS_DATA_DIR: &'static str = "storage/laps";
const RECORDS_FILE: &'static str = "storage/records.bin";
const LAPS_METADATA_FILE: &'static str = "storage/laps.bin";

// pub fn set_root(root: String) {
//     unsafe {
//         ROOT_DIR = root.as_str();
//     }
// }

pub fn ensure_storage_files_created() {
    ensure_folder_created(get_full_path(STORAGE_DIR));
    ensure_folder_created(get_full_path(LAPS_DATA_DIR));
    ensure_file_created(get_full_path(RECORDS_FILE));
    ensure_file_created(get_full_path(LAPS_METADATA_FILE));
}

fn get_full_path(path: &str) -> String {
    let root = unsafe { ROOT_DIR };
    return format!("{}/{}", root, path);
}

pub fn ensure_folder_created(path: String) {
    let folder_path = path.as_str();
    if !Path::new(folder_path).exists() {
        match create_dir(folder_path) {
            Ok(_) => (),
            Err(e) => panic!("failed to create folder, path: {},  e: {}", folder_path, e),
        }
    }
}

pub fn ensure_file_created(path: String) {
    let file_path = path.as_str();
    if !Path::new(file_path).exists() {
        match File::create(file_path) {
            Ok(_) => (),
            Err(e) => panic!("failed to create laps file: {}, e: {}", file_path, e),
        }
    }
}

pub fn persist_record_store(record_store: &RecordStore) {
    let file = File::create(get_full_path(RECORDS_FILE)).expect("failed to create records file");
    bincode::serialize_into(file, &record_store.record_sets)
        .expect("failed to serialise records file");
}

pub fn load_record_store() -> RecordStore {
    let file = File::open(get_full_path(RECORDS_FILE)).expect("failed to open records file");
    match bincode::deserialize_from::<File, HashMap<String, RecordSet>>(file) {
        Ok(x) => RecordStore::new(x),
        Err(_) => RecordStore::new(HashMap::<String, RecordSet>::new()),
    }
}

pub fn load_lap_store() -> LapStore {
    let file = File::open(get_full_path(LAPS_METADATA_FILE)).expect("failed to open records file");
    match bincode::deserialize_from::<File, Vec<LapMetadata>>(file) {
        Ok(x) => LapStore::new(x),
        Err(e) => {
            println!("error opening laps file: {}", e);
            LapStore::new(Vec::new())
        }
    }
}

pub fn store_lap_packets(packets: &Vec<Packet>, metadata: &LapMetadata) {
    let path = format!("{}/{}", get_full_path(LAPS_DATA_DIR), &metadata.identifier);
    let file = File::create(path).unwrap();
    bincode::serialize_into(file, packets).unwrap();
}

pub fn store_metadata(metadata: &Vec<LapMetadata>) {
    let file =
        File::create(get_full_path(LAPS_METADATA_FILE)).expect("failed to create records file");
    bincode::serialize_into(file, &metadata).expect("failed to serialise records file");
}

pub fn get_lap_data(identifier: &str) -> Option<Vec<Packet>> {
    let full_path = format!("{}/{}", get_full_path(LAPS_DATA_DIR), &identifier);
    println!("loading file >> {}", full_path);

    let file = File::open(full_path).expect("failed to open file");
    return bincode::deserialize_from::<File, Vec<Packet>>(file).ok();
}

pub fn get_all_laps_data() -> Vec<Packet> {
    let paths = read_dir(get_full_path(LAPS_DATA_DIR)).unwrap();

    let mut file_paths: Vec<String> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if file_name.ends_with(".bin") {
            file_paths.push(file_name.to_owned());
        }
    }

    file_paths.sort();

    let mut packets = Vec::<Packet>::new();
    for file_path in file_paths {
        let full_path = format!("{}/{}", get_full_path(LAPS_DATA_DIR), file_path);
        println!("loading file >> {}", full_path);

        let file = File::open(full_path).expect("failed to open file");
        let data = bincode::deserialize_from::<File, Vec<Packet>>(file).ok();

        if data.is_some() {
            packets.extend(data.unwrap());
        }
    }

    return packets;
}
