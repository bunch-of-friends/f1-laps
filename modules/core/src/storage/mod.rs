pub(crate) mod models;
pub(crate) mod repository;

use file_system::{self, path_helper::PathHelper};
use std::sync::mpsc;
use storage::models::{LapData, LapHeader};
use storage::repository::Repository;
use udp::Packet;

pub(crate) struct Storage {
    pub path_helper: PathHelper,
    pub lap_headers: Repository<LapHeader>,
    pub lap_data: Repository<LapData>,
}

impl Storage {
    pub fn new(storage_folder_path: &str) -> Storage {
        let path_helper = file_system::initialise(storage_folder_path);
        Storage {
            lap_headers: Repository::<LapHeader>::new(path_helper.get_lap_headers_folder_path()),
            lap_data: Repository::<LapData>::new(path_helper.get_lap_data_folder_path()),
            path_helper: path_helper,
        }
    }

    pub fn store_packets(&self, packets: Vec<Packet>) {
        file_system::store_packets(packets, &self.path_helper);
    }

    pub fn get_all_packets(&self, tx: &mpsc::Sender<Vec<Packet>>) {
        file_system::get_all_packets(&self.path_helper, tx)
    }
}
