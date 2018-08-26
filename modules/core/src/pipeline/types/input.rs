#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tick {
    pub header: Header,
    pub session_info: Option<SessionInfo>,
    pub lap_data: Option<LapData>,
    pub car_motion: Option<CarMotion>,
    pub car_telemetry: Option<CarTelemetry>,
    pub car_status: Option<CarStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub session_uid: u64,
    pub session_time: f32,
    pub player_index: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LapData {
    pub car_position: u8,
    pub last_lap_time: f32,
    pub sector1_time: f32,
    pub sector2_time: f32,
    pub current_sector_number: u8,
    pub current_lap_number: u8,
    pub current_lap_time: f32,
    pub current_lap_distance: f32,
    pub pit_status: u8,
    pub is_lap_valid: bool,
    pub penaltis: u8,
    pub driver_status: u8,
    pub result_status: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CarMotion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub g_force_lateral: f32,
    pub g_force_longitudinal: f32,
    pub g_force_vertical: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionInfo {
    pub weather: u8,
    pub era: u8,
    pub session_type: u8,
    pub track_id: i8,
    pub track_temperature: i8,
    pub air_temperature: i8,
    pub race_laps: u8,
    pub track_lenght: u16,
    pub session_time_left: f32,
    pub session_duration: f32,
    pub is_game_paused: bool,
    pub is_spectating: bool,
    pub is_online_game: bool,
    pub safety_car_status: u8,
    pub marshal_zones_count: u8,
    pub marshal_zones: [MarshalZone; 21],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarshalZone {
    pub zone_start: f32,
    pub flag: i8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CarStatus {
    pub traction_control: u8,
    pub antilock_brakes: u8,
    pub fuel_mix: u8,
    pub front_brake_bias: u8,
    pub pit_limiter_status: u8,
    pub fuel_in_tank: f32,
    pub fuel_capacity: f32,
    pub max_rpm: u16,
    pub max_gears: u8,
    pub is_drs_allowed: bool,
    pub tyres_wear: [u8; 4],
    pub tyre_compound: u8,
    pub tyres_damage: [u8; 4],
    pub front_left_wing_damage: u8,
    pub front_right_wing_damage: u8,
    pub rear_wing_damage: u8,
    pub engine_damage: u8,
    pub gearbox_damage: u8,
    pub exhaust_damage: u8,
    pub flags: i8,
    pub ers_stored: f32,
    pub ers_mode: u8,
    pub ers_harvested_mghu: f32,
    pub ers_harvested_mghh: f32,
    pub ers_deployed: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CarTelemetry {
    pub speed: u16,
    pub throttle: u8,
    pub steer: i8,
    pub brake: u8,
    pub gear: i8,
    pub clutch: u8,
    pub rev_lights_percent: u8,
    pub engine_rpm: u16,
    pub is_drs_open: bool,
    pub brakes_temperature: [u16; 4],
    pub tyres_surface_temperature: [u16; 4],
    pub tyres_inner_temperature: [u16; 4],
    pub engine_temperature: u16,
    pub tyres_pressure: [f32; 4],
}
