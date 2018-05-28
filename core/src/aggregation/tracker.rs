use aggregation::tick::{Lap, Sector, Session};
use udp::packet::Packet;

pub struct Tracker {
    pub current_session: Option<Session>,
    pub current_lap_number: f32,
    pub sector_times: [f32; 3],
    pub current_sector: f32,
    pub current_session_time: f32,
}

impl Tracker {
    pub fn track(&mut self, packet: &Packet) -> (Option<Session>, Option<Sector>, Option<Lap>) {
        let is_current_session = self.is_packet_from_current_session(&packet);
        let is_current_lap = self.is_packet_from_current_lap(&packet, is_current_session);
        let is_current_sector = self.is_packet_from_current_sector(&packet, is_current_lap);

        return (
            self.track_session(&packet, is_current_session),
            self.track_sector(&packet, is_current_sector),
            self.track_lap(&packet, is_current_lap),
        );
    }

    fn is_packet_from_current_session(&self, packet: &Packet) -> bool {
        if self.current_session.is_none() {
            return false;
        }

        // naive check if the packet is coming from the same session
        // a player can be in the same era, same car, same team, same track, but different session -> so this needs improvements
        let unwrapped = self.current_session.unwrap();
        return unwrapped.era == packet.era && unwrapped.session_type == packet.session_type
            && unwrapped.team_id == packet.team_id
            && unwrapped.track_id == packet.track_id;
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
            era: packet.era,
            track_id: packet.track_id,
            team_id: packet.team_id,
            session_type: packet.session_type,
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
        Lap {
            session_time_stamp: packet.time,
            lap_number: packet.lap, // previous lap would be -1, but laps start from 0, so +1 - therefore no adjustment
            lap_time: packet.last_lap_time,
            sector1_time: self.sector_times[0],
            sector2_time: self.sector_times[1],
            sector3_time: self.sector_times[2],
            tyre_compound: packet.tyre_compound,
        }
    }

    fn build_sector_object(&self, packet: &Packet, current_sector: (f32, f32)) -> Sector {
        Sector {
            session_time_stamp: packet.time,
            sector: current_sector.0,
            sector_time: current_sector.1,
            tyre_compound: packet.tyre_compound,
        }
    }

    // also returns just finished sector number and time - to avoid checking the same stuff twice
    fn update_sector_times(&mut self, packet: &Packet) -> (f32, f32) {
        if packet.sector == 0 as f32 {
            let time = packet.last_lap_time - (self.sector_times[0] + self.sector_times[1]);
            self.sector_times[2] = time;
            return (2 as f32, time);
        } else if packet.sector == 1 as f32 {
            self.sector_times[0] = packet.sector1_time;
            return (0 as f32, packet.sector1_time);
        } else if packet.sector == 2 as f32 {
            self.sector_times[1] = packet.sector2_time;
            return (1 as f32, packet.sector2_time);
        } else {
            panic!("unexpected sector number: , {}", packet.sector)
        }
    }
}
