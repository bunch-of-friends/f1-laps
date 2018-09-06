pub mod path_helper;

use bincode;
use std::fs::{create_dir, read_dir, File};
use std::path::Path;

use std::sync::mpsc;
use udp::Packet;

use self::path_helper::PathHelper;

pub(crate) fn initialise(storage_folder_path: &str) -> PathHelper {
    let path_helper = PathHelper::new(storage_folder_path);
    ensure_storage_folders_created(&path_helper);

    build_data_store(&path_helper)
}

pub(crate) fn store_packets(packets: Vec<Packet>, path_helper: &PathHelper) {
    let path = path_helper.get_packets_file_name();
    let file = File::create(&path).unwrap();
    bincode::serialize_into(file, &packets).unwrap();
}

pub(crate) fn get_all_packets(path_helper: &PathHelper, tx: &mpsc::Sender<Vec<Packet>>) {
    let packets_dir = path_helper.get_packets_folder_path();
    let paths = read_dir(packets_dir).unwrap();

    let mut file_names: Vec<String> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if file_name.ends_with(".bin") {
            file_names.push(file_name.to_owned());
        }
    }

    file_names.sort();

    for file_name in file_names {
        let full_path = path_helper.get_packet_file_path(&file_name);

        let file = File::open(full_path).expect("failed to open file");
        let data = bincode::deserialize_from::<File, Vec<Packet>>(file).ok();

        if data.is_some() {
            tx.send(data.unwrap()).ok();
        }
    }
}

fn ensure_storage_folders_created(path_helper: &PathHelper) {
    ensure_folder_created(path_helper.get_storage_folder_path().as_str());
    ensure_folder_created(path_helper.get_packets_folder_path().as_str());
    ensure_folder_created(path_helper.get_lap_headers_folder_path().as_str());
    ensure_folder_created(path_helper.get_lap_data_folder_path().as_str());
}

fn ensure_folder_created(path: &str) {
    if !Path::new(path).exists() {
        match create_dir(path) {
            Ok(_) => (),
            Err(e) => panic!("failed to create folder, path: {},  e: {}", path, e),
        }
    }
}

fn build_data_store(path_helper: &PathHelper) -> PathHelper {
    path_helper.clone()
}
