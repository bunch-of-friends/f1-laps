pub mod data_store;

use self::data_store::DataStore;
use file_system;
use file_system::path_helper;
use std::sync::Mutex;

lazy_static! {
    static ref DATA_STORE: Mutex<DataStore> = Mutex::new(DataStore::new());
}

pub fn initialise(storage_folder_path: &str) {
    let path_helper = file_system::initialise(storage_folder_path);
    DATA_STORE.lock().unwrap().initialise(path_helper)
}

pub fn store_packets(packets: Vec<Vec<u8>>) {
    DATA_STORE.lock().unwrap().store_packets(packets)
}

pub fn get_all_packets() -> Vec<Vec<u8>> {
    DATA_STORE.lock().unwrap().get_all_packets()
}
