use bincode;
use chrono::Local;
use std::fs::{read_dir, File};

use storage::lap::LapMetadata;
use udp::packet::Packet;

pub struct LapStore {
    pub laps_metadata: Option<Vec<LapMetadata>>,
}

impl LapStore {
    pub fn new(laps_metadata: Vec<LapMetadata>) -> LapStore {
        LapStore {
            laps_metadata: Some(laps_metadata),
        }
    }

    pub fn get_all_laps_data(&self) -> Vec<Packet> {
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
        }

        return packets;
    }

    pub fn store_lap(&mut self, packets: Vec<Packet>, metadata: LapMetadata) {
        self.store_lap_metadata(&metadata);
        self.store_lap_packets(&packets, &metadata);
    }

    fn store_lap_metadata(&mut self, metadata: &LapMetadata) {
        let mut laps_metadata = self.laps_metadata.clone();
        let is_empty = laps_metadata.is_none();
        if is_empty {
            laps_metadata = Some(vec![]);
        }
        let mut unwrapped = laps_metadata.unwrap();
        unwrapped.push(metadata.clone());
        self.laps_metadata = Some(unwrapped);

        self.persist_metadata();
    }

    fn store_lap_packets(&self, packets: &Vec<Packet>, metadata: &LapMetadata) {
        let path = format!("storage/laps/{}", &metadata.identifier);
        let file = File::create(path).unwrap();
        bincode::serialize_into(file, &packets).unwrap();
    }

    fn persist_metadata(&self) {
        let file = File::create("storage/laps.bin").expect("failed to create records file");
        bincode::serialize_into(file, &self.laps_metadata)
            .expect("failed to serialise records file");
    }
}
