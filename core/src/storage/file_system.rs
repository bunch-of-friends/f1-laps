use bincode;
use std::fs::{create_dir, read_dir, File};
use std::path::Path;
use storage::lap::LapMetadata;
use storage::lap_store::LapStore;
use storage::path_helper::PathHelper;
use udp::packet::Packet;

pub fn initialise(storage_folder_path: &str) -> LapStore {
    let path_helper = PathHelper::new(storage_folder_path);
    ensure_storage_files_created(&path_helper);

    return load_lap_store(&path_helper);
}

pub fn store_lap_packets(packets: &Vec<Packet>, metadata: &LapMetadata, path_helper: &PathHelper) {
    let path = path_helper.get_laps_data_file_path(&metadata.identifier);
    let file = File::create(path).unwrap();
    bincode::serialize_into(file, packets).unwrap();
}

pub fn store_metadata(metadata: &Vec<LapMetadata>, path_helper: &PathHelper) {
    let path = path_helper.get_laps_metadata_file_path();
    let file = File::create(path).expect("failed to create records file");
    bincode::serialize_into(file, &metadata).expect("failed to serialise records file");
}

pub fn get_lap_data(identifier: &str, path_helper: &PathHelper) -> Option<Vec<Packet>> {
    let path = path_helper.get_laps_data_file_path(identifier);

    println!("loading file >> {}", path);

    let file = File::open(path).expect("failed to open file");
    return bincode::deserialize_from::<File, Vec<Packet>>(file).ok();
}

pub fn get_all_laps_data(path_helper: &PathHelper) -> Vec<Packet> {
    let laps_data_folder = &path_helper.get_laps_data_folder_path();
    let paths = read_dir(laps_data_folder).unwrap();

    let mut file_names: Vec<String> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if file_name.ends_with(".bin") {
            file_names.push(file_name.to_owned());
        }
    }

    file_names.sort();

    let mut packets = Vec::<Packet>::new();
    for file_name in file_names {
        let full_path = path_helper.get_laps_data_file_path(&file_name);
        println!("loading file >> {}", full_path);

        let file = File::open(full_path).expect("failed to open file");
        let data = bincode::deserialize_from::<File, Vec<Packet>>(file).ok();

        if data.is_some() {
            packets.extend(data.unwrap());
        }
    }

    return packets;
}

fn ensure_storage_files_created(path_helper: &PathHelper) {
    ensure_folder_created(path_helper.get_storage_folder_path().as_str());
    ensure_folder_created(path_helper.get_laps_data_folder_path().as_str());
    ensure_file_created(path_helper.get_laps_metadata_file_path().as_str());
}

fn ensure_folder_created(path: &str) {
    if !Path::new(path).exists() {
        match create_dir(path) {
            Ok(_) => (),
            Err(e) => panic!("failed to create folder, path: {},  e: {}", path, e),
        }
    }
}

fn ensure_file_created(path: &str) {
    if !Path::new(path).exists() {
        match File::create(path) {
            Ok(_) => (),
            Err(e) => panic!("failed to create laps file: {}, e: {}", path, e),
        }
    }
}

fn load_lap_store(path_helper: &PathHelper) -> LapStore {
    let path = path_helper.get_laps_metadata_file_path();
    let file = File::open(path).expect("failed to open records file");
    match bincode::deserialize_from::<File, Vec<LapMetadata>>(file) {
        Ok(x) => LapStore::new(x, path_helper.clone()),
        Err(e) => {
            println!("error opening laps file: {}", e);
            LapStore::new(Vec::new(), path_helper.clone())
        }
    }
}
