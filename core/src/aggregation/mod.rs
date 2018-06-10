pub mod tick;
pub mod tracker;

use self::tick::{Lap, LiveData, Tick};
use self::tracker::Tracker;

use udp::packet::Packet;

static mut TRACKER: Tracker = Tracker {
    current_session: None,
    record_tracker: None,
    last_lap: None,
    current_lap_number: -1 as f32,
    lap_packets: None,
    current_sector_times: [-1 as f32, -1 as f32, -1 as f32],
    current_sector: -1 as f32,
    current_session_time: -1 as f32,
};

pub fn process_packet(packet: &Packet, is_replay: bool) -> Option<Tick> {
    if packet.is_spectating == 1 {
        println!("spectating");
        return None;
    }

    let tracking_data = unsafe { TRACKER.track(&packet, is_replay) };
    let live_data = build_live_data(&packet);

    let mut lap_finished: Option<Lap> = None;
    if tracking_data.2.is_some() {
        lap_finished = Some(tracking_data.2.unwrap().0);
    }

    let tick = Tick {
        live_data: live_data,
        session_started: tracking_data.0,
        sector_finished: tracking_data.1,
        lap_finished: lap_finished,
    };

    return Some(tick);
}

pub fn convert_packets(packets: &Vec<Packet>) -> Vec<LiveData> {
    return packets.into_iter().map(|x| build_live_data(x)).collect();
}

fn build_live_data(&packet: &Packet) -> LiveData {
    LiveData {
        current_lap: packet.lap as u8,
        current_lap_time: packet.lap_time,
        current_sector: packet.sector as u8,
        current_speed: packet.speed * 3.6 as f32, // convert mps to kph
        current_gear: (packet.gear as i8) - (1 as i8), //we want reverse to be -1, not 0 as codemasters provide
        current_tyre_compound: packet.tyre_compound as u8,
        is_lap_valid: packet.current_lap_invalid == 0,
        last_lap_time: packet.last_lap_time,
        current_lap_sector1_time: packet.sector1_time,
        current_lap_sector2_time: packet.sector2_time,
        total_session_time: packet.time,
        total_session_distance: packet.total_distance,
        x: packet.x,
        y: packet.y,
        z: packet.z,
        session_time: packet.time,
        session_time_left: packet.session_time_left,
        lap_distance: packet.lap_distance,
        total_distance: packet.total_distance,
        total_laps: packet.total_laps as u8,
        car_position: packet.car_position,
        in_pits: packet.in_pits as u8,
        pit_limiter_status: packet.pit_limiter_status == (1 as u8),
        pit_speed_limit: packet.pit_speed_limit,
        drs: packet.drs == (1 as f32),
        drs_allowed: packet.drs_allowed as i8,
        vehicle_fia_flags: packet.vehicle_fia_flags as i8,
        throttle: packet.throttle,
        steer: packet.steer,
        brake: packet.brake,
        gforce_lat: packet.gforce_lat,
        gforce_lon: packet.gforce_lon,
        gforce_vert: packet.gforce_vert,
        engine_rate: packet.engine_rate,
        rev_lights_percent: packet.rev_lights_percent,
        max_rpm: packet.max_rpm,
        idle_rpm: packet.idle_rpm,
        max_gears: packet.max_gears as u8,
        traction_control: packet.traction_control,
        anti_lock_brakes: packet.anti_lock_brakes,
        front_brake_bias: packet.front_brake_bias,
        fuel_in_tank: packet.fuel_in_tank,
        fuel_capacity: packet.fuel_capacity,
        fuel_mix: packet.fuel_mix,
        engine_temperature: packet.engine_temperature,
        brakes_temperature: packet.brakes_temperature,
        tyres_pressure: packet.tyres_pressure,
        tyres_temperature: packet.tyres_temperature,
        tyres_wear: packet.tyres_wear,
        tyre_compound: packet.tyre_compound,
        tyres_damage: packet.tyres_damage,
        front_left_wing_damage: packet.front_left_wing_damage,
        front_right_wing_damage: packet.front_right_wing_damage,
        rear_wing_damage: packet.rear_wing_damage,
        engine_damage: packet.engine_damage,
        gear_box_damage: packet.gear_box_damage,
        exhaust_damage: packet.exhaust_damage,
        cars_total: packet.cars_total,
        player_car_index: packet.player_car_index,
        car_data: packet.car_data,
    }
}
