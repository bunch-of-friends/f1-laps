use aggregation::tick::{Lap, Sector};
use record_tracking::Record;
use std::collections::HashMap;

pub struct RecordTracker {
    records: HashMap<u8, Record>,
}

#[derive(Debug, Copy, Clone)]
pub struct RecordMarker {
    is_best_ever_personal: bool,
    is_best_ever_compound_personal: bool,
    is_best_session_personal: bool,
    is_best_session_all: bool,
    is_best_session_personal_compound: bool,
    is_best_session_all_compound: bool,
}

impl RecordTracker {
    pub fn new(records: HashMap<u8, Record>) -> RecordTracker {
        RecordTracker { records: records }
    }

    pub fn track_sector_finished(
        &self,
        sector_time: f32,
        sector: u8,
        tyre_compound: u8,
    ) -> RecordMarker {
        // TODO:

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
        &self,
        lap_time: f32,
        sector_times: [f32; 3],
        tyre_compound: u8,
    ) -> RecordMarker {
        // TODO:

        RecordMarker {
            is_best_ever_personal: false,
            is_best_ever_compound_personal: false,
            is_best_session_personal: false,
            is_best_session_all: false,
            is_best_session_personal_compound: false,
            is_best_session_all_compound: false,
        }
    }
}
