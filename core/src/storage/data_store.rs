use lap_metadata::LapMetadata;
use record_tracking::record_tracker::RecordTracker;
use record_tracking::RecordSet;
use storage::file_system;
use storage::path_helper::PathHelper;
use udp::packet::Packet;

pub struct DataStore {
    pub laps_metadata: Option<Vec<LapMetadata>>,
    pub record_set: Option<RecordSet>,
    pub path_helper: Option<PathHelper>,
}

impl DataStore {
    pub fn new(
        laps_metadata: Vec<LapMetadata>,
        record_set: RecordSet,
        path_helper: PathHelper,
    ) -> DataStore {
        DataStore {
            laps_metadata: Some(laps_metadata),
            record_set: Some(record_set),
            path_helper: Some(path_helper),
        }
    }

    pub fn get_all_laps_metadata(&self) -> Vec<LapMetadata> {
        return self.laps_metadata.clone().unwrap();
    }

    pub fn get_lap_data(&self, identifier: &str) -> Option<Vec<Packet>> {
        return file_system::get_lap_data(identifier, &self.path_helper.as_ref().unwrap());
    }

    pub fn get_all_laps_data(&self) -> Vec<Packet> {
        return file_system::get_all_laps_data(&self.path_helper.as_ref().unwrap());
    }

    pub fn store_lap(&mut self, packets: Vec<Packet>, metadata: LapMetadata) {
        self.store_lap_metadata(&metadata);
        file_system::store_lap_packets(&packets, &metadata, &self.path_helper.as_ref().unwrap());
    }

    pub fn get_record_tracker(&self, track_id: u8, era: u16) -> RecordTracker {
        let records = self.record_set.as_ref().unwrap().get_track_records(track_id, era);
        return RecordTracker::new(records);
    }

    fn store_lap_metadata(&mut self, metadata: &LapMetadata) {
        let mut laps_metadata = self.laps_metadata.clone();
        let is_empty = laps_metadata.is_none();
        if is_empty {
            laps_metadata = Some(vec![]);
        }
        let mut unwrapped = laps_metadata.unwrap();
        unwrapped.push(metadata.clone());
        let to_be_stored = unwrapped.clone();

        self.laps_metadata = Some(unwrapped);

        file_system::store_metadata(&to_be_stored, &self.path_helper.as_ref().unwrap());
    }
}
