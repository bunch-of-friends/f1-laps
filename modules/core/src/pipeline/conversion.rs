use pipeline::types::*;
use udp::packet::{Packet, PacketCar};

impl Tick {
    pub fn from_packet(packet: &Packet) -> Tick {
        assert!(packet.lap > 0 as f32);

        let cars: Vec<Car> = packet
            .car_data
            .into_iter()
            .map(|c| Car::from_packet(&c))
            .collect();

        Tick {
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
            is_drs_open: packet.drs == 1 as f32,
            sector_number: (packet.sector as u8) + 1,
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
            is_current_lap_valid: packet.current_lap_invalid != 1 as u8,
            is_spectating: packet.is_spectating == 1 as u8,
            car_index: packet.player_car_index,
            cars_total: packet.cars_total,
            cars: cars,
        }
    }
}

impl Car {
    pub fn from_packet(c: &PacketCar) -> Car {
        Car {
            x: c.world_position[0],
            y: c.world_position[1],
            z: c.world_position[2],
            last_lap_time: c.last_lap_time,
            current_lap_time: c.current_lap_time,
            best_lap_time: c.best_lap_time,
            driver_id: c.driver_id,
            team_id: c.team_id,
            position: c.car_position,
            tyre_compound: c.tyre_compound,
            sector_number: c.sector,
            sector1_time: c.sector1_time,
            sector2_time: c.sector2_time,
            is_current_lap_valid: c.current_lap_invalid != 1,
            penalties: c.penalties,
        }
    }
}

impl Session {
    pub fn from_tick(tick: &Tick) -> Session {
        Session {
            track_id: tick.track_id,
            session_type: tick.session_type,
            team_id: tick.team_id,
            era: tick.era,
        }
    }

    pub fn empty() -> Session {
        Session {
            track_id: 0,
            session_type: 0,
            team_id: 0,
            era: 0,
        }
    }

    pub fn eq(&self, other: &Session) -> bool {
        self.era == other.era
            && self.session_type == other.session_type
            && self.team_id == other.team_id
            && self.track_id == other.track_id
    }
}

impl Lap {
    pub fn from_tick(tick: &Tick) -> Lap {
        Lap {
            lap_number: tick.lap_number,
            sector_times: [tick.sector1_time, tick.sector2_time, 0 as f32],
            lap_time: tick.lap_time,
            is_finished: false,
        }
    }

    pub fn empty() -> Lap {
        Lap {
            lap_number: 0,
            sector_times: [0 as f32; 3],
            lap_time: 0 as f32,
            is_finished: false,
        }
    }

    pub fn finished(s1_t: f32, s2_t: f32, s3_t: f32, lap_t: f32, lap_n: u8) -> Lap {
        assert!(lap_n > 0);
        assert!((s1_t + s2_t + s3_t) == lap_t);

        Lap {
            lap_number: lap_n,
            sector_times: [s1_t, s2_t, s3_t],
            lap_time: lap_t,
            is_finished: true,
        }
    }
}

impl Sector {
    pub fn from_tick(tick: &Tick) -> Sector {
        Sector {
            sector_number: tick.sector_number,
            sector_time: 0 as f32,
            is_finished: false,
        }
    }

    pub fn empty() -> Sector {
        Sector {
            sector_number: 0,
            sector_time: 0 as f32,
            is_finished: false,
        }
    }

    pub fn finished(t: f32, n: u8) -> Sector {
        Sector {
            sector_number: n,
            sector_time: t,
            is_finished: true,
        }
    }
}

impl Position {
    pub fn empty() -> Position {
        Position {
            x: 0 as f32,
            y: 0 as f32,
            z: 0 as f32,
        }
    }

    pub fn from_tick(tick: &Tick) -> Position {
        Position {
            x: tick.x,
            y: tick.y,
            z: tick.z,
        }
    }
}

impl SessionContext {
    pub fn empty() -> SessionContext {
        SessionContext {
            session: Session::empty(),
            lap: Lap::empty(),
            sector: Sector::empty(),
            position: Position::empty(),
        }
    }
}

impl Context {
    pub fn empty() -> Context {
        Context {
            session_context: SessionContext::empty(),
            history_context: HistoryContext {},
        }
    }
}
