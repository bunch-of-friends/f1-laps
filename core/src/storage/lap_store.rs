use std::fs::{read_dir, File};
use bincode;
use chrono::Local;

use udp::packet::Packet;
use storage::lap::LapMetadata;

pub struct LapStore {
    pub laps_metadata: Option<Vec<LapMetadata>>,
}

impl LapStore {
    pub fn new(laps_metadata: Vec<LapMetadata>) -> LapStore {
        LapStore { laps_metadata: Some(laps_metadata) }
    }
}

pub fn store_lap_data(packets: Vec<Packet>, track_id: f32, lap_number: f32) {
    let date = Local::now();
    let path = format!(
        "storage/laps/lap_{}_track-{:02}_L{:03}.bin",
        date.format("%Y-%m-%d-%H-%M-%S-%f"),
        track_id,
        lap_number
    );
    println!("path {}", path);
    let file = File::create(path).unwrap();
    bincode::serialize_into(file, &packets).unwrap();
}

pub fn get_all_laps_data() -> Vec<Packet> {
    let paths = read_dir("storage/laps").unwrap();

    let mut file_paths: Vec<String> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if file_name.ends_with(".bin") && !file_name.ends_with("records.bin") {
            file_paths.push(file_name.to_owned());
        }
    }

    file_paths.sort();

    let mut packets = Vec::<Packet>::new();
    for path in file_paths {
        let full_path = format!("storage/laps/{}", path);
        println!("loading file >> {}", full_path);

        let file = File::open(full_path).expect("failed to open file");
        let data = bincode::deserialize_from::<File, Vec<Packet>>(file).ok();

        if data.is_some() {
            packets.extend(data.unwrap());
        }
    };

    return packets;
}
