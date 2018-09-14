mod file_system;
pub(crate) mod models;
pub(crate) mod repository;

use std::sync::mpsc;
use storage::file_system::path_helper::PathHelper;
use storage::models::{LapTelemetry, LapHeader};
use storage::repository::Repository;
use udp::Packet;

pub(crate) struct Storage {
    pub path_helper: PathHelper,
    pub lap_headers: Repository<LapHeader>,
    pub lap_telemetry: Repository<LapTelemetry>,
}

impl Storage {
    pub fn new(storage_folder_path: &str) -> Storage {
        let path_helper = file_system::initialise(storage_folder_path);
        Storage {
            lap_headers: Repository::<LapHeader>::new(&path_helper.get_lap_headers_folder_path()),
            lap_telemetry: Repository::<LapTelemetry>::new(&path_helper.get_lap_telemetry_folder_path()),
            path_helper,
        }
    }

    pub fn store_packets(&self, packets: &[Packet]) {
        file_system::store_packets(&packets, &self.path_helper);
    }

    pub fn get_all_packets(&self, tx: &mpsc::Sender<Vec<Packet>>) {
        file_system::get_all_packets(&self.path_helper, tx)
    }
}
