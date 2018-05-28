use aggregation::tick::{Lap, Sector, Session};
use udp::packet::Packet;

pub struct Tracker {
    pub current_session: Option<Session>,
    pub current_lap_number: f32,
    pub current_sector: f32,
    pub current_session_time: f32,
}

impl Tracker {
    pub fn track(&mut self, packet: &Packet) -> (Option<Session>, Option<Lap>, Option<Sector>) {
        let is_current_session = self.is_packet_from_current_session(&packet);
        let is_current_lap = self.is_packet_from_current_lap(&packet, is_current_session);
        let is_current_sector = self.is_packet_from_current_sector(&packet, is_current_lap);

        // println!(
        //     "is_current_session: {}, is_current_lap: {}, is_current_sector: {}",
        //     is_current_session, is_current_lap, is_current_sector
        // );

        return (
            self.track_session(&packet, is_current_session),
            self.track_lap(&packet, is_current_lap),
            self.track_sector(&packet, is_current_sector),
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
            self.current_sector = packet.sector;
            return Some(self.build_sector_object(&packet));
        }
    }

    fn build_lap_object(&self, packet: &Packet) -> Lap {
        Lap {
            session_time_stamp: packet.time,
            lap_number: packet.lap - 1 as f32,  //TODO:
            lap_time: packet.last_lap_time,
            sector1_time: 0 as f32, //TODO:
            sector2_time: 0 as f32, //TODO:
            sector3_time: 0 as f32, //TODO:
            tyre_compound: packet.tyre_compound,
        }
    }

    fn build_sector_object(&self, packet: &Packet) -> Sector {
        Sector {
            session_time_stamp: packet.time,
            sector: packet.sector - 1 as f32, //TODO:
            sector_time: 0 as f32,     //TODO:
            tyre_compound: packet.tyre_compound,
        }
    }
}
