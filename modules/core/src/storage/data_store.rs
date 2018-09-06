use sled::{ConfigBuilder, Tree};
use std::sync::mpsc;
use storage::file_system;
use storage::path_helper::PathHelper;
use udp::Packet;

pub(crate) struct DataStore {
    pub path_helper: Option<PathHelper>,
    pub lap_headers: Option<Tree>,
    pub lap_data: Option<Tree>,
}

impl DataStore {
    pub fn new() -> DataStore {
        DataStore {
            path_helper: None,
            lap_headers: None,
            lap_data: None,
        }
    }

    pub fn initialise(&mut self, path_helper: PathHelper) {
        self.lap_headers = Some(build_tree(path_helper.get_lap_headers_folder_path()));
        self.lap_data = Some(build_tree(path_helper.get_lap_data_folder_path()));
        self.path_helper = Some(path_helper);
    }

    pub fn store_packets(&mut self, packets: Vec<Packet>) {
        file_system::store_packets(packets, &self.path_helper.as_ref().unwrap());
    }

    pub fn get_all_packets(&self, tx: &mpsc::Sender<Vec<Packet>>) {
        file_system::get_all_packets(&self.path_helper.as_ref().unwrap(), tx)
    }
}

fn build_tree(path: String) -> Tree {
    let config = ConfigBuilder::new().path(&path).build();

    match Tree::start(config) {
        Ok(t) => t,
        Err(e) => panic!("failed to build tree from path: {:?}, error: {:?}", path, e),
    }
}
