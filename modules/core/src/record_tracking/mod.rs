pub mod record_tracker;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecordSet {
    tracks: HashMap<String, HashMap<u8, Record>>, //key1 -> track_id + era; key2 -> tyre_compound
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

    pub fn update_track_records(
        &mut self,
        track_id: u8,
        era: u16,
        track_records: HashMap<u8, Record>,
    ) {
        let key = self.build_track_record_key(track_id, era);
        self.tracks.insert(key, track_records);
    }

    fn build_track_record_key(&self, track_id: u8, era: u16) -> String {
        return format!("{}_{}", era, track_id);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    pub lap: [f32; 4],
    pub lap_data_identifier: String,
    pub sectors: [f32; 3],
}

impl Record {
    pub fn new() -> Record {
        Record {
            lap: [-1 as f32, -1 as f32, -1 as f32, -1 as f32],
            lap_data_identifier: String::new(),
            sectors: [-1 as f32, -1 as f32, -1 as f32],
        }
    }

    pub fn is_better_sector(&mut self, sector_time: f32, sector: u8) -> bool {
        let record_time = self.sectors[sector as usize];
        let is_better = record_time <= 0 as f32 || record_time > sector_time;
        if is_better {
            self.update_sector(sector_time, sector);
        }
        return is_better;
    }

    pub fn is_better_lap(&mut self, lap: [f32; 4], lap_data_identifier: &str) -> bool {
        let record_time = self.lap[0];
        let is_better = record_time <= 0 as f32 || record_time > lap[0];
        if is_better {
            self.update_lap(lap, lap_data_identifier);
        }
        return is_better;
    }

    fn update_sector(&mut self, sector_time: f32, sector: u8) {
        self.sectors[sector as usize] = sector_time;
    }

    fn update_lap(&mut self, lap: [f32; 4], lap_data_identifier: &str) {
        self.lap = lap;
        self.lap_data_identifier = String::from(lap_data_identifier);
    }
}
