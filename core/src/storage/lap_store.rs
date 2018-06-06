use storage::file_system;
use storage::lap::LapMetadata;
use udp::packet::Packet;

pub struct LapStore {
    pub laps_metadata: Option<Vec<LapMetadata>>,
}

impl LapStore {
    pub fn new(laps_metadata: Vec<LapMetadata>) -> LapStore {
        LapStore {
            laps_metadata: Some(laps_metadata),
        }
    }

    pub fn get_all_laps_metadata(&self) -> Vec<LapMetadata> {
        return self.laps_metadata.clone().unwrap();
    }

    pub fn get_lap_data(&self, identifier: &str) -> Option<Vec<Packet>> {
        return file_system::get_lap_data(identifier);
    }

    pub fn get_all_laps_data(&self) -> Vec<Packet> {
        return file_system::get_all_laps_data();
    }

    pub fn store_lap(&mut self, packets: Vec<Packet>, metadata: LapMetadata) {
        println!("storing lap...");
        self.store_lap_metadata(&metadata);
        file_system::store_lap_packets(&packets, &metadata);
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

        file_system::store_metadata(&to_be_stored);
    }
}
