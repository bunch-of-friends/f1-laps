use pipeline::types::*;
use udp::packet::Packet;

impl InputTick {
    pub fn from_packet(packet: Packet) -> InputTick {
        InputTick {
            session_time: packet.time,
            session_distance: packet.total_distance,
            lap_time: packet.lap_time,
            lap_distance: packet.lap_distance,
            x: packet.x,
            y: packet.y,
            z: packet.z,
            speed: packet.speed,
            throttle: packet.throttle,
            steer: packet.steer,
            brake: packet.brake,
            gear: packet.gear as u8,
            lap_number: packet.lap as u8,
            engine_rate: packet.engine_rate,
            car_position: packet.car_position as u8,
            drs: packet.drs == 1 as f32,
            sector: packet.sector as u8,
            sector1_time: packet.sector1_time,
            sector2_time: packet.sector2_time,
            team_id: packet.team_id as u8,
            total_laps: packet.total_laps as u8,
            last_lap_time: packet.last_lap_time,
            max_gears: packet.max_gears as u8,
            session_type: packet.session_type as u8,
            track_id: packet.track_id as u8,
            vehicle_fia_flags: packet.vehicle_fia_flags as i8,
            era: packet.era as u16,
            tyre_compound: packet.tyre_compound,
            current_lap_invalid: packet.current_lap_invalid == 1 as u8,
            is_spectating: packet.is_spectating == 1 as u8,
            cars_total: packet.cars_total,
        }
    }
}

impl Session {
    pub fn from_input_tick(tick: &InputTick) -> Session {
        Session {
            track_id: tick.track_id,
            session_type: tick.session_type,
            team_id: tick.team_id,
            era: tick.era,
        }
    }

    pub fn eq(&self, other: &Session) -> bool {
        self.era == other.era
            && self.session_type == other.session_type
            && self.team_id == other.team_id
            && self.track_id == other.track_id
    }
}

impl SessionContext {
    pub fn new() -> SessionContext {
        SessionContext {
            session: None,
            lap_number: 0,
            sector: 0,
        }
    }
}
