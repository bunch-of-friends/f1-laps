use models::packet::Car;

#[derive(Debug, Copy, Clone)]
pub struct Tick {
    pub session: Option<Session>,                // new session started
    pub live_data: Option<LiveData>,             // live data update
    pub best_ever_lap: Option<BestLap>,          // best ever lap time achieved
    pub best_ever_sector: Option<BestSector>,    // best ever sector time achieved
    pub best_session_lap: Option<BestLap>,       // best session lap achieved
    pub best_session_sector: Option<BestSector>, // best session sector time achieved
}

#[derive(Debug, Copy, Clone)]
pub struct Session {
    pub track_id: f32,
    pub session_type: f32,
    pub team_id: f32,
    pub era: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct BestLap {
    pub lap_time: f32,
    pub lap_time_previous: f32,

    pub sector1: f32,
    pub sector2: f32,
    pub sector3: f32,

    pub sector1_previous: f32,
    pub sector2_previous: f32,
    pub sector3_previous: f32,

    pub tyre_compound: u8,
    pub is_best_all_compounds: bool, // if false, means it is best for the current compound only
}

#[derive(Debug, Copy, Clone)]
pub struct BestSector {
    pub sector: u8,
    pub time: f32,
    pub time_previous: f32,

    pub tyre_compound: u8,
    pub is_best_all_compounds: bool, // if false, means it is best for the current compound only
}

#[derive(Debug, Copy, Clone)]
pub struct LiveData {
    pub current_lap: i32,
    pub current_lap_time: f32,

    pub current_sector: u8,
    pub current_speed: f32,
    pub current_gear: u8,
    pub current_tyre_compound: u8,

    pub is_lap_valid: bool,

    pub last_lap_time: f32,

    pub last_lap_sector1_time: f32,
    pub last_lap_sector2_time: f32,
    pub last_lap_sector3_time: f32,

    pub current_lap_sector1_time: f32,
    pub current_lap_sector2_time: f32,

    pub total_session_time: f32,
    pub total_session_distance: f32,
    pub total_session_laps: i32,

    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub session_time: f32,
    pub session_time_left: f32,
    pub lap_distance: f32,
    pub total_distance: f32,
    pub total_laps: f32,

    pub car_position: f32,

    pub in_pits: f32,
    pub pit_limiter_status: u8,
    pub pit_speed_limit: u8,

    pub drs: f32,
    pub drs_allowed: f32,
    pub vehicle_fia_flags: f32,

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
    pub max_gears: f32,
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
