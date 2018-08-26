use pipeline::types::*;
use serialisation::f1_2018::packets::*;

impl PacketHeader {
    pub fn to_model(&self) -> Header {
        Header {
            session_uid: self.m_sessionUID,
            session_time: self.m_sessionTime,
            player_index: self.m_playerCarIndex,
        }
    }
}

impl PacketMotionData {
    pub fn to_model(&self) -> CarMotion {
        let data = self.m_carMotionData[self.m_header.m_playerCarIndex as usize];
        CarMotion {
            x: data.m_worldPositionX,
            y: data.m_worldPositionY,
            z: data.m_worldPositionZ,
            g_force_lateral: data.m_gForceLateral,
            g_force_longitudinal: data.m_gForceLongitudinal,
            g_force_vertical: data.m_gForceVertical,
        }
    }
}

impl PacketCarStatusData {
    pub fn to_model(&self) -> CarStatus {
        let data = self.m_carStatusData[self.m_header.m_playerCarIndex as usize];
        CarStatus {
            traction_control: data.m_tractionControl,
            antilock_brakes: data.m_antiLockBrakes,
            fuel_mix: u8,
            front_brake_bias: u8,
            pit_limiter_status: u8,
            fuel_in_tank: f32,
            fuel_capacity: f32,
            max_rpm: u16,
            max_gears: u8,
            is_drs_allowed: bool,
            tyres_wear: [u8; 4],
            tyre_compound: u8,
            tyres_damage: [u8; 4],
            front_left_wing_damage: u8,
            front_right_wing_damage: u8,
            rear_wing_damage: u8,
            engine_damage: u8,
            gearbox_damage: u8,
            exhaust_damage: u8,
            flags: i8,
            ers_stored: f32,
            ers_mode: u8,
            ers_harvested_mghu: f32,
            ers_harvested_mghh: f32,
            ers_deployed: f32,
        }
    }
}

impl PacketCarTelemetryData {
    pub fn to_model(&self) -> CarTelemetry {
        let mut tick = Tick::new(self.m_header.to_header());
        return tick;
    }
}

impl PacketLapData {
    pub fn to_model(&self) -> LapData {
        let mut tick = Tick::new(self.m_header.to_header());
        return tick;
    }
}

impl PacketSessionData {
    pub fn to_tick(&self) -> SessionInfo {
        let mut tick = Tick::new(self.m_header.to_header());
        return tick;
    }
}
