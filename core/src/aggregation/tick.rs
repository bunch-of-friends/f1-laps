use record_tracking::record_tracker::RecordMarker;
use udp::packet::Car;

#[derive(Debug, Copy, Clone)]
pub struct Tick {
    pub session_started: Option<Session>,
    pub lap_finished: Option<Lap>,
    pub sector_finished: Option<Sector>,
    pub live_data: LiveData,
}

#[derive(Debug, Copy, Clone)]
pub struct Session {
    pub track_id: u8,
    pub session_type: u8,
    pub team_id: u16,
    pub era: u16,
    pub session_time_stamp: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Lap {
    pub session_time_stamp: f32,
    pub lap_number: u8,
    pub lap_time: f32,

    pub sector1_time: f32,
    pub sector2_time: f32,
    pub sector3_time: f32,

    pub tyre_compound: u8,

    pub record_marker: RecordMarker,
}

#[derive(Debug, Copy, Clone)]
pub struct Sector {
    pub session_time_stamp: f32,
    pub sector: u8,
    pub sector_time: f32,

    pub tyre_compound: u8,

    pub record_marker: RecordMarker,
}

#[derive(Debug, Copy, Clone)]
pub struct LiveData {
    pub current_lap: u8,
    pub current_lap_time: f32,

    pub current_sector: u8,
    pub current_speed: f32,
    pub current_gear: u8,
    pub current_tyre_compound: u8,

    pub is_lap_valid: bool,

    pub last_lap_time: f32,

    pub current_lap_sector1_time: f32,
    pub current_lap_sector2_time: f32,

    pub total_session_time: f32,
    pub total_session_distance: f32,

    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub session_time: f32,
    pub session_time_left: f32,
    pub lap_distance: f32,
    pub total_distance: f32,
    pub total_laps: u8,

    pub car_position: f32,

    pub in_pits: u8,
    pub pit_limiter_status: bool,
    pub pit_speed_limit: u8,

    pub drs: bool,
    pub drs_allowed: i8,
    pub vehicle_fia_flags: i8,

    pub throttle: f32,
    pub steer: f32,
    pub brake: f32,

    pub gforce_lat: f32,
    pub gforce_lon: f32,
    pub gforce_vert: f32,

    pub engine_rate: f32,
    pub rev_lights_percent: u8,
    pub max_rpm: f32,
    pub idle_rpm: f32,
    pub max_gears: u8,
    pub traction_control: f32,
    pub anti_lock_brakes: f32,
    pub front_brake_bias: u8,

    pub fuel_in_tank: f32,
    pub fuel_capacity: f32,
    pub fuel_mix: u8,

    pub engine_temperature: f32,
    pub brakes_temperature: [f32; 4],
    pub tyres_pressure: [f32; 4],
    pub tyres_temperature: [u8; 4],
    pub tyres_wear: [u8; 4],
    pub tyre_compound: u8,

    pub tyres_damage: [u8; 4],
    pub front_left_wing_damage: u8,
    pub front_right_wing_damage: u8,
    pub rear_wing_damage: u8,
    pub engine_damage: u8,
    pub gear_box_damage: u8,
    pub exhaust_damage: u8,

    pub cars_total: u8,
    pub player_car_index: u8,
    pub car_data: [Car; 20],
}
