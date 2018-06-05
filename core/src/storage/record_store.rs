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
