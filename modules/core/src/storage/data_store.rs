use storage::file_system;
use storage::path_helper::PathHelper;

pub struct DataStore {
    pub path_helper: Option<PathHelper>,
}

impl DataStore {
    pub fn new() -> DataStore {
        DataStore { path_helper: None }
    }

    pub fn initialise(&mut self, path_helper: PathHelper) {
        self.path_helper = Some(path_helper);
    }

    pub fn store_packets(&mut self, packets: Vec<Vec<u8>>) {
        file_system::store_packets(packets, &self.path_helper.as_ref().unwrap());
    }

    pub fn get_all_packets(&self) -> Vec<Vec<u8>> {
        file_system::get_all_packets(&self.path_helper.as_ref().unwrap())
    }
}
