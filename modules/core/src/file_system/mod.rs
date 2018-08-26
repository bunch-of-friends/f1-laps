pub mod path_helper;

use bincode;
use std::fs::{create_dir, File};
use std::path::Path;

use self::path_helper::PathHelper;
use lap_metadata::LapMetadata;
use pipeline::types::CarTelemetry;
use record_tracking::RecordSet;

pub fn initialise(storage_folder_path: &str) -> (Vec<LapMetadata>, RecordSet, PathHelper) {
    let path_helper = PathHelper::new(storage_folder_path);
    ensure_storage_files_created(&path_helper);

    build_data_store(&path_helper)
}

pub fn store_lap_ticks(
    ticks: &Vec<CarTelemetry>,
    metadata: &LapMetadata,
    path_helper: &PathHelper,
) {
    let path = path_helper.get_laps_data_file_path(&metadata.identifier);
    let file = File::create(&path).unwrap();
    bincode::serialize_into(file, ticks).unwrap();
}

pub fn store_laps_metadata(metadata: &Vec<LapMetadata>, path_helper: &PathHelper) {
    let path = path_helper.get_laps_metadata_file_path();
    let file = File::create(path).expect("failed to create laps metadata file");
    bincode::serialize_into(file, metadata).expect("failed to serialise laps metadata file");
}

pub fn store_records(records: &RecordSet, path_helper: &PathHelper) {
    let path = path_helper.get_records_file_path();
    let file = File::create(path).expect("failed to create records file");
    bincode::serialize_into(file, records).expect("failed to serialise records file");
}

pub fn get_lap_data(identifier: &str, path_helper: &PathHelper) -> Option<Vec<CarTelemetry>> {
    let path = path_helper.get_laps_data_file_path(identifier);

    println!("loading file >> {}", path);

    let file = File::open(path).expect("failed to open file");
    bincode::deserialize_from::<File, Vec<CarTelemetry>>(file).ok()
}

fn ensure_storage_files_created(path_helper: &PathHelper) {
    ensure_folder_created(path_helper.get_storage_folder_path().as_str());
    ensure_folder_created(path_helper.get_packets_data_folder_path().as_str());
    ensure_folder_created(path_helper.get_laps_data_folder_path().as_str());
    ensure_file_created(path_helper.get_laps_metadata_file_path().as_str());
    ensure_file_created(path_helper.get_records_file_path().as_str());
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

fn build_data_store(path_helper: &PathHelper) -> (Vec<LapMetadata>, RecordSet, PathHelper) {
    let lap_metadata = match load_lap_metadata(path_helper) {
        Ok(x) => x,
        Err(_) => Vec::new(),
    };

    let records = match load_records(path_helper) {
        Ok(x) => x,
        Err(_) => RecordSet::new(),
    };

    (lap_metadata, records, path_helper.clone())
}

fn load_lap_metadata(
    path_helper: &PathHelper,
) -> Result<Vec<LapMetadata>, Box<bincode::ErrorKind>> {
    let path = path_helper.get_laps_metadata_file_path();
    let file = File::open(path).expect("failed to open laps metadata file");
    bincode::deserialize_from::<File, Vec<LapMetadata>>(file)
}

fn load_records(path_helper: &PathHelper) -> Result<RecordSet, Box<bincode::ErrorKind>> {
    let path = path_helper.get_records_file_path();
    let file = File::open(path).expect("failed to open records file");
    bincode::deserialize_from::<File, RecordSet>(file)
}
