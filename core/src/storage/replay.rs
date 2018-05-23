use std::fs::{read_dir, File};

use std::sync::mpsc;

use bincode;
use chrono::Local;

use models::api::Tick;
use models::packet::Packet;
use aggregation::process_packet;

//TODO: sort out repeated string constants here and across the storage mod files

pub fn store_replay_data(packets: Vec<Packet>) {
    let date = Local::now();
    let path = format!(
        "storage/test_storage_{}.bin",
        date.format("%Y-%m-%d-%H-%M-%S-%f")
    );
    println!("path {}", path);
    let file = File::create(path).unwrap();
    bincode::serialize_into(file, &packets).unwrap();
}

pub fn get_replay_data(tx: mpsc::Sender<Tick>) {
    let paths = read_dir("storage/").unwrap();

    let mut file_paths: Vec<String> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if file_name.ends_with(".bin") && !file_name.ends_with("records.bin") {
            file_paths.push(file_name.to_owned());
        }
    }

    file_paths.sort();

    for path in file_paths {
        let full_path = format!("storage/{}", path);

        let file = File::open(full_path).expect("failed to open file");
        let data = bincode::deserialize_from::<File, Vec<Packet>>(file).ok();

        if data.is_none() {
            continue;
        }

        for packet in data.unwrap() {
            let tick = process_packet(packet);

            if tick.is_some() {
                tx.send(tick.unwrap())
                    .expect("failed to update the main thread")
            }
        }
    }
}
