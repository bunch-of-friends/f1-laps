#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Packet {
    pub time: f32,
    pub lap_time: f32,
    pub lap_distance: f32,
    pub total_distance: f32,
    pub x: f32,             // World space position
    pub y: f32,             // World space position
    pub z: f32,             // World space position
    pub speed: f32,         // Speed of car in M/S
    pub xv: f32,            // Velocity in world space
    pub yv: f32,            // Velocity in world space
    pub zv: f32,            // Velocity in world space
    pub xr: f32,            // World space right direction
    pub yr: f32,            // World space right direction
    pub zr: f32,            // World space right direction
    pub xd: f32,            // World space forward direction
    pub yd: f32,            // World space forward direction
    pub zd: f32,            // World space forward direction
    pub susp_pos: [f32; 4], // Note: All wheel arrays have the order:
    pub susp_vel: [f32; 4], // RL, RR, FL, FR
    pub wheel_speed: [f32; 4],
    pub throttle: f32,
    pub steer: f32,
    pub brake: f32,
    pub clutch: f32,
    pub gear: f32,
    pub gforce_lat: f32,
    pub gforce_lon: f32,
    pub lap: f32,
    pub engine_rate: f32,
    pub sli_pro_native_support: f32,  // SLI Pro support
    pub car_position: f32,            // car race position
    pub kers_level: f32,              // kers energy left
    pub kers_max_level: f32,          // kers maximum energy
    pub drs: f32,                     // 0 = off, 1 = on
    pub traction_control: f32,        // 0 (off) - 2 (high)
    pub anti_lock_brakes: f32,        // 0 (off) - 1 (on)
    pub fuel_in_tank: f32,            // current fuel mass
    pub fuel_capacity: f32,           // fuel capacity
    pub in_pits: f32,                 // 0 = none, 1 = pitting, 2 = in pit area
    pub sector: f32,                  // 0 = sector1, 1 = sector2, 2 = sector3
    pub sector1_time: f32,            // time of sector1 (or 0)
    pub sector2_time: f32,            // time of sector2 (or 0)
    pub brakes_temperature: [f32; 4], // brakes temperature (centigrade)
    pub tyres_pressure: [f32; 4],     // tyres pressure PSI
    pub team_id: f32,                 // team ID
    pub total_laps: f32,              // total number of laps in this race
    pub track_size: f32,              // track size meters
    pub last_lap_time: f32,           // last lap time
    pub max_rpm: f32,                 // cars max RPM, at which point the rev limiter will kick in
    pub idle_rpm: f32,                // cars idle RPM
    pub max_gears: f32,               // maximum number of gears
    pub session_type: f32,            // 0 = unknown, 1 = practice, 2 = qualifying, 3 = race
    pub drs_allowed: f32,             // 0 = not allowed, 1 = allowed, -1 = invalid / unknown
    pub track_id: f32,                // -1 for unknown, 0-21 for tracks
    pub vehicle_fia_flags: f32, // -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red
    pub era: f32,               // era, 2017 (modern) or 1980 (classic)
    pub engine_temperature: f32, // engine temperature (centigrade)
    pub gforce_vert: f32,       // vertical g-force component
    pub ang_vel_x: f32,         // angular velocity x-component
    pub ang_vel_y: f32,         // angular velocity y-component
    pub ang_vel_z: f32,         // angular velocity z-component
    pub tyres_temperature: [u8; 4], // tyres temperature (centigrade)
    pub tyres_wear: [u8; 4],    // tyre wear percentage
    pub tyre_compound: u8, // compound of tyre – 0 = ultra soft, 1 = super soft, 2 = soft, 3 = medium, 4 = hard, 5 = inter, 6 = wet
    pub front_brake_bias: u8, // front brake bias (percentage)
    pub fuel_mix: u8,      // fuel mix - 0 = lean, 1 = standard, 2 = rich, 3 = max
    pub current_lap_invalid: u8, // current lap invalid - 0 = valid, 1 = invalid
    pub tyres_damage: [u8; 4], // tyre damage (percentage)
    pub front_left_wing_damage: u8, // front left wing damage (percentage)
    pub front_right_wing_damage: u8, // front right wing damage (percentage)
    pub rear_wing_damage: u8, // rear wing damage (percentage)
    pub engine_damage: u8, // engine damage (percentage)
    pub gear_box_damage: u8, // gear box damage (percentage)
    pub exhaust_damage: u8, // exhaust damage (percentage)
    pub pit_limiter_status: u8, // pit limiter status – 0 = off, 1 = on
    pub pit_speed_limit: u8, // pit speed limit in mph
    pub session_time_left: f32, // NEW: time left in session in seconds
    pub rev_lights_percent: u8, // NEW: rev lights indicator (percentage)
    pub is_spectating: u8, // NEW: whether the player is spectating
    pub spectator_car_index: u8, // NEW: index of the car being spectated

    // Car data
    pub cars_total: u8,              // number of cars in data
    pub player_car_index: u8,        // index of player's car in the array
    pub car_data: [PacketCar; 20],         // data for all cars on track
    pub yaw: f32,                    // NEW (v1.8)
    pub pitch: f32,                  // NEW (v1.8)
    pub roll: f32,                   // NEW (v1.8)
    pub x_local_velocity: f32,       // NEW (v1.8) Velocity in local space
    pub y_local_velocity: f32,       // NEW (v1.8) Velocity in local space
    pub z_local_velocity: f32,       // NEW (v1.8) Velocity in local space
    pub susp_acceleration: [f32; 4], // NEW (v1.8) RL, RR, FL, FR
    pub ang_acc_x: f32,              // NEW (v1.8) angular acceleration x-component
    pub ang_acc_y: f32,              // NEW (v1.8) angular acceleration y-component
    pub ang_acc_z: f32,              // NEW (v1.8) angular acceleration z-component
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PacketCar {
    pub world_position: [f32; 3], // world co-ordinates of vehicle
    pub last_lap_time: f32,
    pub current_lap_time: f32,
    pub best_lap_time: f32,
    pub sector1_time: f32,
    pub sector2_time: f32,
    pub lap_distance: f32,
    pub driver_id: u8,
    pub team_id: u8,
    pub car_position: u8, // UPDATED: track positions of vehicle
    pub current_lap_num: u8,
    pub tyre_compound: u8, // compound of tyre – 0 = ultra soft, 1 = super soft, 2 = soft, 3 = medium, 4 = hard, 5 = inter, 6 = wet
    pub in_pits: u8,       // 0 = none, 1 = pitting, 2 = in pit area
    pub sector: u8,        // 0 = sector1, 1 = sector2, 2 = sector3
    pub current_lap_invalid: u8, // current lap invalid - 0 = valid, 1 = invalid
    pub penalties: u8,     // NEW: accumulated time penalties in seconds to be added
}
