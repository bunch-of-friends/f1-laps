use lap_metadata::LapMetadata;
use record_tracking::record_tracker::RecordTracker;
use record_tracking::RecordSet;
use storage::file_system;
use storage::path_helper::PathHelper;

pub struct DataStore {
    pub laps_metadata: Option<Vec<LapMetadata>>,
    pub record_set: Option<RecordSet>,
    pub path_helper: Option<PathHelper>,
}

impl DataStore {
    pub fn new() -> DataStore {
        DataStore {
            laps_metadata: None,
            record_set: None,
            path_helper: None,
        }
    }

    pub fn initialise(
        &mut self,
        laps_metadata: Vec<LapMetadata>,
        record_set: RecordSet,
        path_helper: PathHelper,
    ) {
        self.laps_metadata = Some(laps_metadata);
        self.record_set = Some(record_set);
        self.path_helper = Some(path_helper);
    }

    pub fn get_all_laps_metadata(&self) -> Vec<LapMetadata> {
        return self.laps_metadata.clone().unwrap();
    }

    pub fn get_all_records(&self) -> RecordSet {
        return self.record_set.clone().unwrap();
    }

    // pub fn get_lap_data(&self, identifier: &str) -> Option<Vec<CarTelemetry>> {
    //     return file_system::get_lap_data(identifier, &self.path_helper.as_ref().unwrap());
    // }

    // pub fn store_lap(&mut self, ticks: Vec<CarTelemetry>, metadata: &LapMetadata) {
    //     self.store_lap_metadata(metadata);
    //     file_system::store_lap_ticks(&ticks, metadata, &self.path_helper.as_ref().unwrap());
    // }

    pub fn get_record_tracker(&self, track_id: u8, era: u16) -> RecordTracker {
        let records = self
            .record_set
            .as_ref()
            .unwrap()
            .get_track_records(track_id, era);
        return RecordTracker::new(track_id, era, records);
    }

    pub fn store_records(&mut self, record_tracker: &RecordTracker) {
        let unwrapped = self.record_set.as_mut().unwrap();
        unwrapped.update_track_records(
            record_tracker.track_id,
            record_tracker.era,
            record_tracker.get_records(),
        );

        file_system::store_records(&unwrapped, &self.path_helper.as_ref().unwrap());
    }

    fn store_lap_metadata(&mut self, metadata: &LapMetadata) {
        let unwrapped = self.laps_metadata.as_mut().unwrap();
        unwrapped.push(metadata.clone());

        file_system::store_laps_metadata(&unwrapped, &self.path_helper.as_ref().unwrap());
    }
}
