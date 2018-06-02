use std::collections::HashMap;
use storage::record::RecordSet;

pub struct RecordStore {
    pub record_sets: Option<HashMap<String, RecordSet>>,
}

impl RecordStore {
    pub fn new(record_sets: HashMap<String, RecordSet>) -> RecordStore {
        RecordStore {
            record_sets: Some(record_sets),
        }
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
