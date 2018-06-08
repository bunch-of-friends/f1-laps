const LAPS_DATA_DIR: &'static str = "laps";
const LAPS_METADATA_FILE: &'static str = "laps.bin";

pub struct PathHelper {
    root_folder: String,
}

impl PathHelper {
    pub fn new(base_folder: &str) -> PathHelper {
        PathHelper {
            root_folder: String::from(base_folder),
        }
    }

    pub fn clone(&self) -> PathHelper {
        PathHelper {
            root_folder: self.root_folder.clone(),
        }
    }

    pub fn get_full_path(&self, path: &str) -> String {
        return format!("{}/{}", self.root_folder, path);
    }

    pub fn get_storage_folder_path(&self) -> String {
        return self.root_folder.clone();
    }

    pub fn get_laps_data_folder_path(&self) -> String {
        return self.get_full_path(LAPS_DATA_DIR);
    }

    pub fn get_laps_metadata_file_path(&self) -> String {
        return self.get_full_path(LAPS_METADATA_FILE);
    }

    pub fn get_laps_data_file_path(&self, identifier: &str) -> String {
        return format!("{}/{}", self.get_laps_data_folder_path(), identifier);
    }
}
