pub mod record_tracker;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecordSet {
    tracks: HashMap<String, HashMap<u8, Record>>, //key1 -> track_id + era; key2 -> tyre_compound
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    pub lap: [f32; 4],
    pub sector1_time: f32,
    pub sector2_time: f32,
    pub sector3_time: f32,
}

impl RecordSet {
    pub fn new() -> RecordSet {
        RecordSet {
            tracks: HashMap::new(),
        }
    }

    pub fn get_track_records(&self, track_id: u8, era: u16) -> HashMap<u8, Record> {
        let key = self.build_track_record_key(track_id, era);
        if self.tracks.contains_key(&key) {
            return self.tracks.get(&key).unwrap().clone();
        }
        return HashMap::new();
    }

    pub fn update_track_records(&mut self, track_id: u8, era: u16, track_records: HashMap<u8, Record>) {
        let key = self.build_track_record_key(track_id, era);
        self.tracks.insert(key, track_records);
    }

    fn build_track_record_key(&self, track_id: u8, era: u16) -> String {
        return format!("{}{}", era, track_id);
    }
}
