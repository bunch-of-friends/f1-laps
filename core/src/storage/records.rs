use bincode;
use std::collections::HashMap;
use std::fs::File;

use storage::record_set::RecordSet;

pub fn persist_record_store(record_store: RecordStore) {
    let file = File::create("storage/records.bin").expect("failed to create records file");
    let hash_map = record_store.get_hash_map();
    bincode::serialize_into(file, &hash_map).expect("failed to serialise records file");
}

pub fn get_records_store() -> RecordStore {
    let file = File::open("storage/records.bin").expect("failed to open records file");
    match bincode::deserialize_from::<File, HashMap<String, RecordSet>>(file) {
        Ok(x) => RecordStore::new(x),
        Err(_) => {
            RecordStore::new(HashMap::<String, RecordSet>::new())
        }
    }
}

pub struct RecordStore {
    hash_map: HashMap<String, RecordSet>,
}

impl RecordStore {
    pub fn new(hash_map: HashMap<String, RecordSet>) -> RecordStore {
        RecordStore { hash_map: hash_map }
    }

    pub fn get_hash_map(&self) -> HashMap<String, RecordSet> {
        self.hash_map.clone()
    }
}

// let era = 2017;
// let track_id = 22;

// let record_set = RecordSet {
//     records: [Record {
//         best_lap_time: [LapRecord {
//             lap_time: 0.0,
//             sector1_time: 0.0,
//             sector2_time: 0.0,
//             sector3_time: 0.0,
//         }; 8],
//         best_sector1_time: [0.0; 8],
//         best_sector2_time: [0.0; 8],
//         best_sector3_time: [0.0; 8],
//     }; 5],
// };

// let mut map = HashMap::new();
// map.insert(format!("{}-{}", era, track_id), record_set);
