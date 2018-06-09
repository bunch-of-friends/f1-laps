use aggregation::tick::{Lap, Sector, Session};
use lap_metadata::LapMetadata;
use record_tracking::record_tracker::RecordTracker;
use std::thread;
use storage;
use udp::packet::Packet;

pub struct Tracker {
    pub current_session: Option<Session>,
    pub record_tracker: Option<RecordTracker>,
    pub last_lap: Option<Lap>,

    pub current_lap_number: f32,
    pub current_sector_times: [f32; 3],
    pub current_sector: f32,
    pub current_session_time: f32,

    pub lap_packets: Option<Vec<Packet>>,
}

impl Tracker {
    pub fn track(
        &mut self,
        packet: &Packet,
        should_store_packets: bool,
    ) -> (Option<Session>, Option<Sector>, Option<Lap>) {
        let is_first_packet = self.current_session.is_none();

        let is_current_session = self.is_packet_from_current_session(&packet);
        let is_current_lap = self.is_packet_from_current_lap(&packet, is_current_session);
        let is_current_sector = self.is_packet_from_current_sector(&packet, is_current_lap);

        let started_session = self.track_session(&packet, is_current_session);

        if started_session.is_some() {
            let s = started_session.as_ref().unwrap();
            self.record_tracker = Some(storage::get_record_tracker(s.track_id, s.era))
        }

        let finished_sector = self.track_sector(&packet, is_current_sector);
        let finished_lap = self.track_lap(&packet, is_current_lap);

        // if finished_sector.is_some() {
        //     self.record_tracker
        //         .as_ref()
        //         .unwrap()
        //         .track_sector_finished(finished_sector.as_ref().unwrap());
        // }

        if finished_lap.is_some() {
            self.last_lap = finished_lap;

            // self.record_tracker
            //     .as_ref()
            //     .unwrap()
            //     .track_lap_finished(finished_lap.as_ref().unwrap());
        }

        self.store_packet(packet, should_store_packets, is_current_lap);

        if is_first_packet {
            return (started_session, None, None);
        } else {
            return (started_session, finished_sector, finished_lap);
        }
    }

    fn is_packet_from_current_session(&self, packet: &Packet) -> bool {
        if self.current_session.is_none() {
            return false;
        }

        // naive check if the packet is coming from the same session
        // a player can be in the same era, same car, same team, same track, but different session -> so this needs improvements
        let unwrapped = self.current_session.unwrap();
        return unwrapped.era == (packet.era as u16)
            && unwrapped.session_type == (packet.session_type as u8)
            && unwrapped.team_id == (packet.team_id as u16)
            && unwrapped.track_id == (packet.track_id as u8);
    }

    fn is_packet_from_current_lap(&self, packet: &Packet, is_current_session: bool) -> bool {
        return is_current_session && (self.current_lap_number == packet.lap);
    }

    fn is_packet_from_current_sector(&self, packet: &Packet, is_current_lap: bool) -> bool {
        return is_current_lap && (self.current_sector == packet.sector);
    }

    fn track_session(&mut self, packet: &Packet, is_current_session: bool) -> Option<Session> {
        if is_current_session {
            return None;
        };

        let session = Session {
            era: packet.era as u16,
            track_id: packet.track_id as u8,
            team_id: packet.team_id as u16,
            session_type: packet.session_type as u8,
            session_time_stamp: packet.time,
        };
        self.current_session_time = packet.time;
        self.current_lap_number = packet.lap;
        self.current_sector = packet.sector;
        self.current_session = Some(session);
        return Some(session);
    }

    fn track_lap(&mut self, packet: &Packet, is_current_lap: bool) -> Option<Lap> {
        if is_current_lap {
            return None;
        } else {
            self.current_lap_number = packet.lap;
            return Some(self.build_lap_object(&packet));
        }
    }

    fn store_packet(&mut self, packet: &Packet, should_store_packets: bool, is_current_lap: bool) {
        let mut lap_packets = self.lap_packets.clone();
        let is_empty = lap_packets.is_none();
        if is_empty {
            lap_packets = Some(vec![]);
        }

        let mut unwrapped = lap_packets.unwrap();
        unwrapped.push(packet.clone());

        if !is_empty && !is_current_lap {
            if should_store_packets && self.has_all_sector_times() {
                let session = self.current_session.unwrap();
                let lap = self.last_lap.unwrap();
                let packets_to_store = unwrapped.clone();
                thread::spawn(move || {
                    let metadata = LapMetadata::new(&lap, &session);
                    storage::store_lap(packets_to_store, metadata);
                });
            }
            unwrapped = vec![];
        }

        self.lap_packets = Some(unwrapped);
    }

    fn has_all_sector_times(&self) -> bool {
        return (self.current_sector_times[0] > 0 as f32)
            && (self.current_sector_times[1] > 0 as f32)
            && (self.current_sector_times[2] > 0 as f32);
    }

    fn track_sector(&mut self, packet: &Packet, is_current_sector: bool) -> Option<Sector> {
        if is_current_sector {
            return None;
        } else {
            let finished_sector = self.update_sector_times(&packet);
            self.current_sector = packet.sector;
            return Some(self.build_sector_object(&packet, finished_sector));
        }
    }

    fn build_lap_object(&self, packet: &Packet) -> Lap {
        let lap_number = self.get_previous_lap_number(packet.lap as u8); // as current packet is already from the newly started lap
        let lap_time = packet.last_lap_time;
        let sector1_time = self.current_sector_times[0];
        let sector2_time = self.current_sector_times[1];
        let sector3_time = self.current_sector_times[2];
        let tyre_compound = packet.tyre_compound;

        let record_marker = self.record_tracker.as_ref().unwrap().track_lap_finished(
            lap_time,
            [sector1_time, sector2_time, sector3_time],
            tyre_compound,
        );

        Lap {
            session_time_stamp: packet.time,
            lap_number: lap_number,
            lap_time: lap_time,
            sector1_time: sector1_time,
            sector2_time: sector2_time,
            sector3_time: sector3_time,
            tyre_compound: tyre_compound,
            record_marker: record_marker,
        }
    }

    fn get_previous_lap_number(&self, current_lap_number: u8) -> u8 {
        if current_lap_number > 0 {
            return current_lap_number - 1;
        } else {
            panic!("how is this even possible??");
        }
    }

    fn build_sector_object(&self, packet: &Packet, current_sector: (u8, f32)) -> Sector {
        let record_marker = self.record_tracker.as_ref().unwrap().track_sector_finished(
            current_sector.1,
            current_sector.0,
            packet.tyre_compound,
        );

        Sector {
            session_time_stamp: packet.time,
            sector: current_sector.0,
            sector_time: current_sector.1,
            tyre_compound: packet.tyre_compound,
            record_marker: record_marker,
        }
    }

    // also returns just finished sector number and time - to avoid checking the same stuff twice
    fn update_sector_times(&mut self, packet: &Packet) -> (u8, f32) {
        if packet.sector == 0 as f32 {
            let time = packet.last_lap_time
                - (self.current_sector_times[0] + self.current_sector_times[1]);
            self.current_sector_times[2] = time;
            return (2 as u8, time);
        } else if packet.sector == 1 as f32 {
            self.current_sector_times[0] = packet.sector1_time;
            return (0 as u8, packet.sector1_time);
        } else if packet.sector == 2 as f32 {
            self.current_sector_times[1] = packet.sector2_time;
            return (1 as u8, packet.sector2_time);
        } else {
            panic!("unexpected sector number: , {}", packet.sector)
        }
    }
}
