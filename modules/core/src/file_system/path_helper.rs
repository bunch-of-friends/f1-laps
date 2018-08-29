use chrono::Utc;

use std::clone::Clone;

const PACKETS_DIR: &'static str = "packets";
const LAPS_METADATA_FILE: &'static str = "laps.bin";
const RECORDS_FILE: &'static str = "records.bin";

pub struct PathHelper {
    root_folder: String,
}

impl PathHelper {
    pub fn new(base_folder: &str) -> PathHelper {
        PathHelper {
            root_folder: String::from(base_folder),
        }
    }

    pub fn get_full_path(&self, path: &str) -> String {
        format!("{}/{}", self.root_folder, path)
    }

    pub fn get_storage_folder_path(&self) -> String {
        self.root_folder.clone()
    }

    pub fn get_packets_folder_path(&self) -> String {
        self.get_full_path(PACKETS_DIR)
    }

    pub fn get_laps_metadata_file_path(&self) -> String {
        self.get_full_path(LAPS_METADATA_FILE)
    }

    pub fn get_records_file_path(&self) -> String {
        self.get_full_path(RECORDS_FILE)
    }

    pub fn get_packets_file_name(&self) -> String {
        format!(
            "{}/{}.bin",
            self.get_packets_folder_path(),
            Utc::now().date().format("%Y-%m-%d-%H-%M-%S-%f")
        )
    }
}

impl Clone for PathHelper {
    fn clone(&self) -> PathHelper {
        PathHelper {
            root_folder: self.root_folder.clone(),
        }
    }
}
