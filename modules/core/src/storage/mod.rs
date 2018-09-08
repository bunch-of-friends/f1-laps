pub mod data_store;

use std::sync::Mutex;

use self::data_store::DataStore;
use file_system;
use file_system::path_helper;
use std::sync::mpsc;
use udp::Packet;

lazy_static! {
    static ref DATA_STORE: Mutex<DataStore> = Mutex::new(DataStore::new());
}

pub(crate) fn initialise(storage_folder_path: &str) {
    let path_helper = file_system::initialise(storage_folder_path);
    DATA_STORE.lock().unwrap().initialise(path_helper)
}

pub(crate) fn store_packets(packets: Vec<Packet>) {
    DATA_STORE.lock().unwrap().store_packets(packets)
}

pub(crate) fn get_all_packets(tx: &mpsc::Sender<Vec<Packet>>) {
    DATA_STORE.lock().unwrap().get_all_packets(tx)
}
