use udp::packet::Packet;
use storage::record_set::Record;
use aggregation::tick::Session;

pub struct SessionTracker {
    pub current_session: Option<Session>,
    pub session_record: Option<Record>,
    pub track_record: Option<Record>,
}

impl SessionTracker {
    pub fn track(&mut self, packet: Packet) -> Option<Session> {
        if self.is_packet_from_current_session(packet) {
            return None;
        };

        let session = Session {
            era: packet.era,
            track_id: packet.track_id,
            team_id: packet.team_id,
            session_type: packet.session_type,
        };

        self.current_session = Some(session);
        Some(session)
    }

    fn is_packet_from_current_session(&self, packet: Packet) -> bool {
        let session = self.current_session;
        if session.is_none() {
            return false;
        }

        // naive check if the packet is coming from the same session
        // a player can be in the same era, same car, same team, same track, but different session -> so this needs improvements
        let unwrapped = session.unwrap();
        return unwrapped.era == packet.era && unwrapped.session_type == packet.session_type
            && unwrapped.team_id == packet.team_id
            && unwrapped.track_id == packet.track_id;
    }
}
