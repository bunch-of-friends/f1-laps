use storage::file_system;
use storage::path_helper::PathHelper;
use udp::Packet;
use std::sync::mpsc;

pub(crate) struct DataStore {
    pub path_helper: Option<PathHelper>,
}

impl DataStore {
    pub fn new() -> DataStore {
        DataStore { path_helper: None }
    }

    pub fn initialise(&mut self, path_helper: PathHelper) {
        self.path_helper = Some(path_helper);
    }

    pub fn store_packets(&mut self, packets: Vec<Packet>) {
        file_system::store_packets(packets, &self.path_helper.as_ref().unwrap());
    }

    pub fn get_all_packets(&self, tx: &mpsc::Sender<Vec<Packet>>) {
        file_system::get_all_packets(&self.path_helper.as_ref().unwrap(), tx)
    }
}
