pub mod session_tracker;

use self::session_tracker::SessionTracker;
use models::packet::Packet;
use models::api::{BestLap, BestSector, LiveData, Session, Tick};
use storage;

static mut SESSION_TRACKER: SessionTracker = SessionTracker {
    current_session: None,
    session_record: None,
    track_record: None,
};

static mut RECORDS_STORE: Option<storage::records::RecordStore> = None;

pub fn process_packet(packet: Packet) -> Option<Tick> {
    let session: Option<Session> = unsafe { SESSION_TRACKER.track(packet) };

    let live_data = LiveData {
        current_lap: packet.lap as i32,
        current_lap_time: packet.lap_time,
        current_sector: packet.sector as u8,
        current_speed: packet.speed,
        current_gear: packet.gear as u8,
        current_tyre_compound: packet.tyre_compound as u8,
        is_lap_valid: packet.current_lap_invalid == 0,
        last_lap_time: packet.last_lap_time,
        last_lap_sector1_time: 0.0, //TODO
        last_lap_sector2_time: 0.0, //TODO
        last_lap_sector3_time: 0.0, //TODO
        current_lap_sector1_time: packet.sector1_time,
        current_lap_sector2_time: packet.sector2_time,
        total_session_time: packet.time,
        total_session_distance: packet.total_distance,
        total_session_laps: 0, //TODO
        x: packet.x,
        y: packet.y,
        z: packet.z,
        session_time: packet.time,
        session_time_left: packet.session_time_left,
        lap_distance: packet.lap_distance,
        total_distance: packet.total_distance,
        total_laps: packet.total_laps,
        car_position: packet.car_position,
        in_pits: packet.in_pits,
        pit_limiter_status: packet.pit_limiter_status,
        pit_speed_limit: packet.pit_speed_limit,
        drs: packet.drs,
        drs_allowed: packet.drs_allowed,
        vehicle_fia_flags: packet.vehicle_fia_flags,
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
        max_gears: packet.max_gears,
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
    };

    // for testing only -------- vv

    // let best_session_lap = BestLap {
    //     lap_time: 60.00,
    //     lap_time_previous: 62.222,
    //     sector1: 10.0,
    //     sector2: 20.0,
    //     sector3: 30.0,
    //     sector1_previous: 12.222,
    //     sector2_previous: 20.0,
    //     sector3_previous: 30.0,
    //     tyre_compound: 2,
    //     is_best_all_compounds: false,
    // };

    // let best_session_sector = BestSector {
    //     sector: 1,
    //     time: 12.12,
    //     time_previous: 24.24,
    //     tyre_compound: 0,
    //     is_best_all_compounds: true,
    // };

    // let best_ever_lap = BestLap {
    //     lap_time: 60.00,
    //     lap_time_previous: 62.222,
    //     sector1: 10.0,
    //     sector2: 20.0,
    //     sector3: 30.0,
    //     sector1_previous: 12.222,
    //     sector2_previous: 20.0,
    //     sector3_previous: 30.0,
    //     tyre_compound: 2,
    //     is_best_all_compounds: false,
    // };

    // let best_ever_sector = BestSector {
    //     sector: 2,
    //     time: 123.123,
    //     time_previous: 124.124,
    //     tyre_compound: 1,
    //     is_best_all_compounds: false,
    // };

    // for testing only -------- ^^

    let tick = Tick {
        live_data: Some(live_data),
        session: session,
        best_session_lap: None,    // Some(best_session_lap),
        best_session_sector: None, //Some(best_session_sector),
        best_ever_lap: None,       // Some(best_ever_lap),
        best_ever_sector: None,    // Some(best_ever_sector),
    };

    return Some(tick);
}

pub fn preload_records() {
    let has_records_store = unsafe {
        RECORDS_STORE.is_some()
    };

    if has_records_store {
        return;
    }

    let records_store = storage::records::get_records_store();
    unsafe {
        RECORDS_STORE = Some(records_store);
    }
}
