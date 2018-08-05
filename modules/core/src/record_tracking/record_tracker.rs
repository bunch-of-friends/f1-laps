use record_tracking::Record;
use std::collections::HashMap;

pub struct RecordTracker {
    pub track_id: u8,
    pub era: u16,
    records: HashMap<u8, Record>,
}

impl RecordTracker {
    pub fn new(track_id: u8, era: u16, records: HashMap<u8, Record>) -> RecordTracker {
        RecordTracker {
            track_id: track_id,
            era: era,
            records: records,
        }
    }

    pub fn track_sector_finished(
        &mut self,
        _sector_time: f32,
        _sector: u8,
        _tyre_compound: u8,
    ) -> RecordMarker {
        // let results = self.is_best_ever_sector(sector_time, sector, tyre_compound);

        RecordMarker {
            is_best_ever_personal: false,
            is_best_ever_compound_personal: false,
            is_best_session_personal: false,
            is_best_session_all: false,
            is_best_session_personal_compound: false,
            is_best_session_all_compound: false,
        }
    }

    pub fn track_lap_finished(
        &mut self,
        is_valid_lap: bool,
        lap: [f32; 4],
        tyre_compound: u8,
        lap_data_identifier: &str,
    ) -> RecordMarker {
        if !is_valid_lap {
            return RecordMarker::no_records();
        }

        let results = self.is_best_ever_lap(lap, tyre_compound, lap_data_identifier);

        RecordMarker {
            is_best_ever_personal: results.0,
            is_best_ever_compound_personal: results.1,
            is_best_session_personal: false,
            is_best_session_all: false,
            is_best_session_personal_compound: false,
            is_best_session_all_compound: false,
        }
    }

    pub fn get_records(&self) -> HashMap<u8, Record> {
        return self.records.clone();
    }

    fn is_best_ever_lap(
        &mut self,
        lap: [f32; 4],
        tyre_compound: u8,
        lap_data_identifier: &str,
    ) -> (bool, bool) {
        let mut best_overall = true;
        let mut best_compound = true;

        if !self.records.contains_key(&tyre_compound) {
            let record = Record::new();
            self.records.insert(tyre_compound, record);
        }

        let records = self.records.clone();

        for (compound, _) in records.iter() {
            let is_better = self.records
                .get_mut(compound)
                .unwrap()
                .is_better_lap(lap, lap_data_identifier);
            if !is_better {
                best_overall = false;
                if compound == &tyre_compound {
                    best_compound = false;
                    return (best_overall, best_compound); //at this point no point iterating, it's false/false
                }
            }
        }

        if best_overall {
            println!("best overall lap >> {:?}", lap);
        } else if best_compound {
            println!("best lap on this compound >> {:?}", lap);
        }

        return (best_overall, best_compound);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RecordMarker {
    pub is_best_ever_personal: bool,
    pub is_best_ever_compound_personal: bool,
    pub is_best_session_personal: bool,
    pub is_best_session_all: bool,
    pub is_best_session_personal_compound: bool,
    pub is_best_session_all_compound: bool,
}

impl RecordMarker {
    pub fn no_records() -> RecordMarker {
        RecordMarker {
            is_best_ever_personal: false,
            is_best_ever_compound_personal: false,
            is_best_session_personal: false,
            is_best_session_all: false,
            is_best_session_personal_compound: false,
            is_best_session_all_compound: false,
        }
    }

    pub fn has_any_best_ever_records(&self) -> bool {
        return self.is_best_ever_personal || self.is_best_ever_compound_personal;
    }
}
