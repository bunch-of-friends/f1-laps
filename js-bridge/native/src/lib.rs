#[macro_use]
extern crate neon;
extern crate f1_laps_core;

use f1_laps_core::aggregation::tick::{Lap, LiveData, Sector, Session};
use f1_laps_core::udp::packet::Car;
use neon::js::{JsArray, JsBoolean, JsNumber, JsObject, JsUndefined, Object};
use neon::mem::Handle;
use neon::scope;
use neon::vm::{Call, JsResult};

fn initialise(_call: Call) -> JsResult<JsUndefined> {
    f1_laps_core::initialise();

    Ok(JsUndefined::new())
}

fn start_listening(call: Call) -> JsResult<JsUndefined> {
    let port_handle = call.arguments.get(call.scope, 0).unwrap();
    let should_store_replay_handle = call.arguments.get(call.scope, 1).unwrap();

    let port = port_handle
        .downcast::<JsNumber>()
        .expect("failed to downcast port argument")
        .value() as i32;

    let should_store_replay = should_store_replay_handle
        .downcast::<JsBoolean>()
        .expect("failed to downcast shouldStoreReplay argument")
        .value() as bool;

    f1_laps_core::start_listening(port, should_store_replay);

    Ok(JsUndefined::new())
}

fn replay_all_laps(_call: Call) -> JsResult<JsUndefined> {
    f1_laps_core::replay_all_laps();

    Ok(JsUndefined::new())
}

#[allow(unused_must_use)]
fn get_next_tick(call: Call) -> JsResult<JsObject> {
    let tick_option = f1_laps_core::get_next_tick();

    let scope = call.scope;
    let object = JsObject::new(scope);

    if tick_option.is_none() {
        return Ok(object);
    }

    let tick = tick_option.unwrap();

    object.set("liveData", build_live_data_js_object(scope, tick.live_data));

    if let Some(session) = tick.session_started {
        object.set("sessionStarted", build_session_js_object(scope, session));
    }

    if let Some(lap) = tick.lap_finished {
        object.set("lapFinished", build_lap_js_object(scope, lap));
    }

    if let Some(sector) = tick.sector_finished {
        object.set("sectorFinished", build_sector_js_object(scope, sector));
    }

    Ok(object)
}

#[allow(unused_must_use)]
fn build_session_js_object<'a>(
    scope: &mut scope::RootScope<'a>,
    session: Session,
) -> Handle<'a, JsObject> {
    let object = JsObject::new(scope);

    object.set(
        "sessionTimeStamp",
        JsNumber::new(scope, session.session_time_stamp as f64),
    );
    object.set("era", JsNumber::new(scope, session.era as f64));
    object.set("trackId", JsNumber::new(scope, session.track_id as f64));
    object.set("teamId", JsNumber::new(scope, session.team_id as f64));
    object.set(
        "sessionType",
        JsNumber::new(scope, session.session_type as f64),
    );

    object
}

#[allow(unused_must_use)]
fn build_live_data_js_object<'a>(
    scope: &mut scope::RootScope<'a>,
    live_data: LiveData,
) -> Handle<'a, JsObject> {
    let object = JsObject::new(scope);

    object.set(
        "currentLap",
        JsNumber::new(scope, live_data.current_lap as f64),
    );
    object.set(
        "currentLapTime",
        JsNumber::new(scope, live_data.current_lap_time as f64),
    );
    object.set(
        "currentSector",
        JsNumber::new(scope, live_data.current_sector as f64),
    );
    object.set(
        "currentSpeed",
        JsNumber::new(scope, live_data.current_speed as f64),
    );
    object.set(
        "currentGear",
        JsNumber::new(scope, live_data.current_gear as f64),
    );
    object.set(
        "currentTyreCompound",
        JsNumber::new(scope, live_data.current_tyre_compound as f64),
    );
    object.set("isLapValid", JsBoolean::new(scope, live_data.is_lap_valid));
    object.set(
        "lastLapTime",
        JsNumber::new(scope, live_data.last_lap_time as f64),
    );
    object.set(
        "currentLap_sector1Time",
        JsNumber::new(scope, live_data.current_lap_sector1_time as f64),
    );
    object.set(
        "currentLapSector1Time",
        JsNumber::new(scope, live_data.current_lap_sector1_time as f64),
    );
    object.set(
        "totalSessionTime",
        JsNumber::new(scope, live_data.total_session_time as f64),
    );
    object.set(
        "totalSessionDistance",
        JsNumber::new(scope, live_data.total_session_distance as f64),
    );

    object.set("x", JsNumber::new(scope, live_data.x as f64));
    object.set("y", JsNumber::new(scope, live_data.y as f64));
    object.set("z", JsNumber::new(scope, live_data.z as f64));
    object.set(
        "sessionTime",
        JsNumber::new(scope, live_data.session_time as f64),
    );
    object.set(
        "sessionTimeLeft",
        JsNumber::new(scope, live_data.session_time_left as f64),
    );
    object.set(
        "lapDistance",
        JsNumber::new(scope, live_data.lap_distance as f64),
    );
    object.set(
        "totalDistance",
        JsNumber::new(scope, live_data.total_distance as f64),
    );
    object.set(
        "totalLaps",
        JsNumber::new(scope, live_data.total_laps as f64),
    );
    object.set(
        "carPosition",
        JsNumber::new(scope, live_data.car_position as f64),
    );
    object.set("inPits", JsNumber::new(scope, live_data.in_pits as f64));
    object.set(
        "pitLimiterStatus",
        JsNumber::new(scope, live_data.pit_limiter_status as f64),
    );
    object.set(
        "pitSpeedLimit",
        JsNumber::new(scope, live_data.pit_speed_limit as f64),
    );
    object.set("drs", JsNumber::new(scope, live_data.drs as f64));
    object.set(
        "drsAllowed",
        JsNumber::new(scope, live_data.drs_allowed as f64),
    );
    object.set(
        "vehicleFiaFlags",
        JsNumber::new(scope, live_data.vehicle_fia_flags as f64),
    );
    object.set("throttle", JsNumber::new(scope, live_data.throttle as f64));
    object.set("steer", JsNumber::new(scope, live_data.steer as f64));
    object.set("brake", JsNumber::new(scope, live_data.brake as f64));
    object.set(
        "gforceLat",
        JsNumber::new(scope, live_data.gforce_lat as f64),
    );
    object.set(
        "gforceLon",
        JsNumber::new(scope, live_data.gforce_lon as f64),
    );
    object.set(
        "gforceVert",
        JsNumber::new(scope, live_data.gforce_vert as f64),
    );
    object.set(
        "engineRate",
        JsNumber::new(scope, live_data.engine_rate as f64),
    );
    object.set(
        "revLightsPercent",
        JsNumber::new(scope, live_data.rev_lights_percent as f64),
    );
    object.set("maxRpm", JsNumber::new(scope, live_data.max_rpm as f64));
    object.set("idleRpm", JsNumber::new(scope, live_data.idle_rpm as f64));
    object.set("maxGears", JsNumber::new(scope, live_data.max_gears as f64));
    object.set(
        "tractionControl",
        JsNumber::new(scope, live_data.traction_control as f64),
    );
    object.set(
        "antiLockBrakes",
        JsNumber::new(scope, live_data.anti_lock_brakes as f64),
    );
    object.set(
        "frontBrakeBias",
        JsNumber::new(scope, live_data.front_brake_bias as f64),
    );
    object.set(
        "fuelInTank",
        JsNumber::new(scope, live_data.fuel_in_tank as f64),
    );
    object.set(
        "fuelCapacity",
        JsNumber::new(scope, live_data.fuel_capacity as f64),
    );
    object.set("fuelMix", JsNumber::new(scope, live_data.fuel_mix as f64));
    object.set(
        "engineTemperature",
        JsNumber::new(scope, live_data.engine_temperature as f64),
    );

    let brakes_temperature = JsArray::new(scope, 4);
    brakes_temperature.set(
        0,
        JsNumber::new(scope, live_data.brakes_temperature[0] as f64),
    );
    brakes_temperature.set(
        1,
        JsNumber::new(scope, live_data.brakes_temperature[1] as f64),
    );
    brakes_temperature.set(
        2,
        JsNumber::new(scope, live_data.brakes_temperature[2] as f64),
    );
    brakes_temperature.set(
        3,
        JsNumber::new(scope, live_data.brakes_temperature[3] as f64),
    );
    object.set("brakesTemperature", brakes_temperature);

    let tyres_pressure = JsArray::new(scope, 4);
    tyres_pressure.set(0, JsNumber::new(scope, live_data.tyres_pressure[0] as f64));
    tyres_pressure.set(1, JsNumber::new(scope, live_data.tyres_pressure[1] as f64));
    tyres_pressure.set(2, JsNumber::new(scope, live_data.tyres_pressure[2] as f64));
    tyres_pressure.set(3, JsNumber::new(scope, live_data.tyres_pressure[3] as f64));
    object.set("tyresPressure", tyres_pressure);

    let tyres_temperature = JsArray::new(scope, 4);
    tyres_temperature.set(
        0,
        JsNumber::new(scope, live_data.tyres_temperature[0] as f64),
    );
    tyres_temperature.set(
        1,
        JsNumber::new(scope, live_data.tyres_temperature[1] as f64),
    );
    tyres_temperature.set(
        2,
        JsNumber::new(scope, live_data.tyres_temperature[2] as f64),
    );
    tyres_temperature.set(
        3,
        JsNumber::new(scope, live_data.tyres_temperature[3] as f64),
    );
    object.set("tyresTemperature", tyres_temperature);

    let tyres_wear = JsArray::new(scope, 4);
    tyres_wear.set(0, JsNumber::new(scope, live_data.tyres_wear[0] as f64));
    tyres_wear.set(1, JsNumber::new(scope, live_data.tyres_wear[1] as f64));
    tyres_wear.set(2, JsNumber::new(scope, live_data.tyres_wear[2] as f64));
    tyres_wear.set(3, JsNumber::new(scope, live_data.tyres_wear[3] as f64));
    object.set("tyresWear", tyres_wear);

    object.set(
        "tyreCompound",
        JsNumber::new(scope, live_data.tyre_compound as f64),
    );

    let tyres_damage = JsArray::new(scope, 4);
    tyres_damage.set(0, JsNumber::new(scope, live_data.tyres_damage[0] as f64));
    tyres_damage.set(1, JsNumber::new(scope, live_data.tyres_damage[1] as f64));
    tyres_damage.set(2, JsNumber::new(scope, live_data.tyres_damage[2] as f64));
    tyres_damage.set(3, JsNumber::new(scope, live_data.tyres_damage[3] as f64));
    object.set("tyresDamage", tyres_damage);

    object.set(
        "frontLeftWingDamage",
        JsNumber::new(scope, live_data.front_left_wing_damage as f64),
    );
    object.set(
        "frontRightWingDamage",
        JsNumber::new(scope, live_data.front_right_wing_damage as f64),
    );
    object.set(
        "rearWingDamage",
        JsNumber::new(scope, live_data.rear_wing_damage as f64),
    );
    object.set(
        "engineDamage",
        JsNumber::new(scope, live_data.engine_damage as f64),
    );
    object.set(
        "gearBoxDamage",
        JsNumber::new(scope, live_data.gear_box_damage as f64),
    );
    object.set(
        "exhaustDamage",
        JsNumber::new(scope, live_data.exhaust_damage as f64),
    );
    object.set(
        "carsTotal",
        JsNumber::new(scope, live_data.cars_total as f64),
    );
    object.set(
        "playerCarIndex",
        JsNumber::new(scope, live_data.player_car_index as f64),
    );

    let car_data = JsArray::new(scope, 20);
    car_data.set(0, build_car_js_object(scope, live_data.car_data[0]));
    car_data.set(1, build_car_js_object(scope, live_data.car_data[1]));
    car_data.set(2, build_car_js_object(scope, live_data.car_data[2]));
    car_data.set(3, build_car_js_object(scope, live_data.car_data[3]));
    car_data.set(4, build_car_js_object(scope, live_data.car_data[4]));
    car_data.set(5, build_car_js_object(scope, live_data.car_data[5]));
    car_data.set(6, build_car_js_object(scope, live_data.car_data[6]));
    car_data.set(7, build_car_js_object(scope, live_data.car_data[7]));
    car_data.set(8, build_car_js_object(scope, live_data.car_data[8]));
    car_data.set(9, build_car_js_object(scope, live_data.car_data[9]));
    car_data.set(10, build_car_js_object(scope, live_data.car_data[10]));
    car_data.set(11, build_car_js_object(scope, live_data.car_data[11]));
    car_data.set(12, build_car_js_object(scope, live_data.car_data[12]));
    car_data.set(13, build_car_js_object(scope, live_data.car_data[13]));
    car_data.set(14, build_car_js_object(scope, live_data.car_data[14]));
    car_data.set(15, build_car_js_object(scope, live_data.car_data[15]));
    car_data.set(16, build_car_js_object(scope, live_data.car_data[16]));
    car_data.set(17, build_car_js_object(scope, live_data.car_data[17]));
    car_data.set(18, build_car_js_object(scope, live_data.car_data[18]));
    car_data.set(19, build_car_js_object(scope, live_data.car_data[19]));
    object.set("carData", car_data);

    object
}

#[allow(unused_must_use)]
fn build_car_js_object<'a>(scope: &mut scope::RootScope<'a>, car: Car) -> Handle<'a, JsObject> {
    let object = JsObject::new(scope);

    let world_position = JsArray::new(scope, 3);
    world_position.set(0, JsNumber::new(scope, car.world_position[0] as f64));
    world_position.set(1, JsNumber::new(scope, car.world_position[1] as f64));
    world_position.set(2, JsNumber::new(scope, car.world_position[2] as f64));

    object.set("worldPosition", world_position);
    object.set(
        "lastLapTime",
        JsNumber::new(scope, car.last_lap_time as f64),
    );
    object.set(
        "currentLapTime",
        JsNumber::new(scope, car.current_lap_time as f64),
    );
    object.set(
        "bestLapTime",
        JsNumber::new(scope, car.best_lap_time as f64),
    );
    object.set("sector1Time", JsNumber::new(scope, car.sector1_time as f64));
    object.set("sector2Time", JsNumber::new(scope, car.sector2_time as f64));
    object.set("lapDistance", JsNumber::new(scope, car.lap_distance as f64));
    object.set("driverId", JsNumber::new(scope, car.driver_id as f64));
    object.set("teamId", JsNumber::new(scope, car.team_id as f64));
    object.set("carPosition", JsNumber::new(scope, car.car_position as f64));

    object.set(
        "currentLapNum",
        JsNumber::new(scope, car.current_lap_num as f64),
    );
    object.set("inPits", JsNumber::new(scope, car.in_pits as f64));
    object.set("sector", JsNumber::new(scope, car.sector as f64));
    object.set(
        "currentLapInvalid",
        JsNumber::new(scope, car.current_lap_invalid as f64),
    );
    object.set("penalties", JsNumber::new(scope, car.penalties as f64));

    object
}

#[allow(unused_must_use)]
fn build_lap_js_object<'a>(scope: &mut scope::RootScope<'a>, lap: Lap) -> Handle<'a, JsObject> {
    let object = JsObject::new(scope);

    object.set(
        "sessionTimeStamp",
        JsNumber::new(scope, lap.session_time_stamp as f64),
    );

    object.set("lapNumber", JsNumber::new(scope, lap.lap_number as f64));
    object.set("lapTime", JsNumber::new(scope, lap.lap_time as f64));
    object.set("sector1Time", JsNumber::new(scope, lap.sector1_time as f64));
    object.set("sector2Time", JsNumber::new(scope, lap.sector2_time as f64));
    object.set("sector3Time", JsNumber::new(scope, lap.sector3_time as f64));

    object.set(
        "tyreCompound",
        JsNumber::new(scope, lap.tyre_compound as f64),
    );

    object
}

#[allow(unused_must_use)]
fn build_sector_js_object<'a>(
    scope: &mut scope::RootScope<'a>,
    sector: Sector,
) -> Handle<'a, JsObject> {
    let object = JsObject::new(scope);

    object.set(
        "sessionTimeStamp",
        JsNumber::new(scope, sector.session_time_stamp as f64),
    );
    object.set("sector", JsNumber::new(scope, sector.sector as f64));
    object.set(
        "sectorTime",
        JsNumber::new(scope, sector.sector_time as f64),
    );
    object.set(
        "tyreCompound",
        JsNumber::new(scope, sector.tyre_compound as f64),
    );

    object
}

register_module!(m, {
    m.export("initialise", initialise)
        .expect("failed to export initialise");
    m.export("getNextTick", get_next_tick)
        .expect("failed to export getNextTick");
    m.export("startListening", start_listening)
        .expect("failed to export startListening");
    m.export("replayAllLaps", replay_all_laps)
        .expect("failed to export replayAllLaps");
    Ok(())
});
