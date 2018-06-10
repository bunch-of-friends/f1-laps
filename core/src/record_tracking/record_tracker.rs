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
        sector_time: f32,
        sector: u8,
        tyre_compound: u8,
    ) -> RecordMarker {
        let results = self.is_best_ever_sector(sector_time, sector, tyre_compound);

        RecordMarker {
            is_best_ever_personal: results.0,
            is_best_ever_compound_personal: results.1,
            is_best_session_personal: false,
            is_best_session_all: false,
            is_best_session_personal_compound: false,
            is_best_session_all_compound: false,
        }
    }

    pub fn track_lap_finished(&mut self, lap: [f32; 4], tyre_compound: u8, lap_data_identifier: &str) -> RecordMarker {
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

    fn is_best_ever_sector(
        &mut self,
        sector_time: f32,
        sector: u8,
        tyre_compound: u8,
    ) -> (bool, bool) {
        let mut best_overall = true;
        let mut best_compound = true;

        let records = self.records.clone();

        for (compound, _) in records.iter() {
            let is_better = self.records
                .get_mut(compound)
                .unwrap()
                .is_better_sector(sector_time, sector);
            if !is_better {
                best_overall = false;
                if compound == &tyre_compound {
                    best_compound = false;
                    return (best_overall, best_compound); //at this point no point iterating, it's false/false
                }
            }
        }
        return (best_overall, best_compound);
    }

    fn is_best_ever_lap(&mut self, lap: [f32; 4], tyre_compound: u8, lap_data_identifier: &str) -> (bool, bool) {
        let mut best_overall = true;
        let mut best_compound = true;

        let records = self.records.clone();

        for (compound, _) in records.iter() {
            let is_better = self.records.get_mut(compound).unwrap().is_better_lap(lap, lap_data_identifier);
            if !is_better {
                best_overall = false;
                if compound == &tyre_compound {
                    best_compound = false;
                    return (best_overall, best_compound); //at this point no point iterating, it's false/false
                }
            }
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
    pub fn has_any_best_ever_records(&self) -> bool {
        return self.is_best_ever_personal || self.is_best_ever_compound_personal;
    }
}
