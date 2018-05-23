pub mod replay;
pub mod records;

use std::path::Path;
use std::fs::{create_dir, File};

pub fn ensure_storage_files_created() {
    let folder_path = "storage";
    if !Path::new(folder_path).exists() {
        match create_dir(folder_path) {
            Ok(_) => (),
            Err(e) => panic!("failed to create folder, path: {},  e: {}", folder_path, e),
        }
    }

    let records_path = "storage/records.bin";
    if !Path::new(records_path).exists() {
        match File::create(records_path) {
            Ok(_) => (),
            Err(e) => panic!("failed to create records file: {}", e),
        }
    }
}
