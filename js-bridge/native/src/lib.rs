#[macro_use]
extern crate neon;
extern crate f1_laps_core;

pub mod arr_helpers;
pub mod obj_helpers;

use arr_helpers as ah;
use f1_laps_core::aggregation::tick::{Lap, LiveData, Sector, Session};
use f1_laps_core::lap_metadata::LapMetadata;
use f1_laps_core::record_tracking::record_tracker::RecordMarker;
use f1_laps_core::udp::packet::Car;
use neon::prelude::*;
use obj_helpers as oh;

fn initialise(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let storage_folder_path = cx.argument::<JsString>(0)?.value();

    f1_laps_core::initialise(storage_folder_path);

    Ok(JsUndefined::new())
}

fn start_listening(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let port = cx.argument::<JsNumber>(0)?.value() as i32;

    f1_laps_core::start_listening(port);

    Ok(JsUndefined::new())
}

fn replay_all_laps(_cx: FunctionContext) -> JsResult<JsUndefined> {
    f1_laps_core::replay_all_laps();

    Ok(JsUndefined::new())
}

fn get_next_tick(mut cx: FunctionContext) -> JsResult<JsObject> {
    let tick_option = f1_laps_core::get_next_tick();

    let object = cx.empty_object();

    if tick_option.is_none() {
        return Ok(object);
    }

    let tick = tick_option.unwrap();

    let d_obj = build_d_js_object(&mut cx, &tick.live_data);
    oh::set_obj_prop(&mut cx, &object, "liveData", d_obj)?;

    // if let Some(session) = tick.session_started {
    //     obj.set(cx, "sessionStarted", build_session_js_object(scope, &session));
    // }

    // if let Some(lap) = tick.lap_finished {
    //     obj.set(cx, "lapFinished", build_lap_js_object(scope, &lap));
    // }

    // if let Some(sector) = tick.sector_finished {
    //     obj.set(cx, "sectorFinished", build_sector_js_object(scope, &sector));
    // }

    Ok(object)
}

// #[allow(unused_must_use)]
// fn get_lap_data(cx: FunctionContext) -> JsResult<JsArray> {
//     let identifier = cx.argument::<JsString>(0)?.value();

//     let data = f1_laps_core::get_lap_data(identifier);
//     let array = JsArray::new(cx.scope, data.len() as u32);

//     let mut index = 0;
//     for item in data.iter() {
//         let js_object = build_d_js_object(cx.scope, item);
//         array.set(index, js_object);
//         index += 1;
//     }

//     Ok(array)
// }

// #[allow(unused_must_use)]
// fn get_all_laps_metadata(cx: FunctionContext) -> JsResult<JsArray> {
//     let metadata = f1_laps_core::get_all_laps_metadata();
//     let array = JsArray::new(cx.scope, metadata.len() as u32);
//     let mut index = 0;
//     for item in metadata.iter() {
//         let js_object = build_lap_metadata_js_object(cx.scope, item);
//         array.set(index, js_object);
//         index += 1;
//     }

//     Ok(array)
// }

// #[allow(unused_must_use)]
// fn replay_lap(cx: FunctionContext) -> JsResult<JsUndefined> {
//     let identifier_handle = cx.arguments.get(cx.scope, 0).unwrap();

//     let identifier = identifier_handle
//         .downcast::<JsString>()
//         .expect("failed to downcast identifier argument")
//         .value();

//     f1_laps_core::replay_lap(identifier);

//     Ok(JsUndefined::new())
// }

// #[allow(unused_must_use)]
// fn build_lap_metadata_js_object<'a>(
//     scope: &mut scope::RootScope<'a>,
//     metadata: &LapMetadata,
// ) -> Handle<'a, JsObject> {
//     let object = JsObject::new(scope);

//     obj.set(cx,
//         "identifier",
//         JsString::new(scope, metadata.identifier.as_str()).unwrap(),
//     );
//     obj.set(cx,
//         "recordedDate",
//         JsString::new(scope, metadata.recorded_date.as_str()).unwrap(),
//     );
//     obj.set(cx, "trackId", cx.number( metadata.track_id as f64));
//     obj.set(cx, "teamId", cx.number( metadata.team_id as f64));
//     obj.set(cx, "era", cx.number( metadata.era as f64));
//     obj.set(cx,
//         "tyreCompound",
//         cx.number( metadata.tyre_compound as f64),
//     );
//     obj.set(cx,
//         "sessionType",
//         cx.number( metadata.session_type as f64),
//     );
//     obj.set(cx,
//         "lapNumber",
//         cx.number( metadata.lap_number as f64),
//     );
//     obj.set(cx, "lapTime", cx.number( metadata.lap_time as f64));
//     obj.set(cx,
//         "note",
//         JsString::new(scope, metadata.note.as_str()).unwrap(),
//     );

//     let sector_times = JsArray::new(scope, 3);
//     sector_times.set(0, cx.number( metadata.sector_times[0] as f64));
//     sector_times.set(1, cx.number( metadata.sector_times[1] as f64));
//     sector_times.set(2, cx.number( metadata.sector_times[2] as f64));
//     obj.set(cx, "sectorTimes", sector_times);

//     object
// }

// #[allow(unused_must_use)]
// fn build_session_js_object<'a>(
//     scope: &mut scope::RootScope<'a>,
//     session: &Session,
// ) -> Handle<'a, JsObject> {
//     let object = JsObject::new(scope);

//     obj.set(cx,
//         "sessionTimeStamp",
//         cx.number( session.session_time_stamp as f64),
//     );
//     obj.set(cx, "era", cx.number( session.era as f64));
//     obj.set(cx, "trackId", cx.number( session.track_id as f64));
//     obj.set(cx, "teamId", cx.number( session.team_id as f64));
//     obj.set(cx,
//         "sessionType",
//         cx.number( session.session_type as f64),
//     );

//     object
// }

fn build_d_js_object<'a>(
    cx: &mut FunctionContext<'a>,
    d: &LiveData,
) -> NeonResult<Handle<'a, JsObject>> {
    let obj = cx.empty_object();

    oh::set_num_prop(cx, &obj, "currentLap", d.current_lap as f64)?;
    oh::set_num_prop(cx, &obj, "currentLapTime", d.current_lap_time as f64)?;
    oh::set_num_prop(cx, &obj, "currentSector", d.current_sector as f64)?;
    oh::set_num_prop(cx, &obj, "currentSpeed", d.current_speed as f64)?;
    oh::set_num_prop(cx, &obj, "currentGear", d.current_gear as f64)?;
    oh::set_num_prop(
        cx,
        &obj,
        "currentTyreCompound",
        d.current_tyre_compound as f64,
    )?;
    oh::set_bool_prop(cx, &obj, "isLapValid", d.is_lap_valid)?;
    oh::set_num_prop(cx, &obj, "lastLapTime", d.last_lap_time as f64)?;
    oh::set_num_prop(
        cx,
        &obj,
        "currentLapSector1Time",
        d.current_lap_sector1_time as f64,
    )?;
    oh::set_num_prop(
        cx,
        &obj,
        "currentLapSector2Time",
        d.current_lap_sector2_time as f64,
    )?;
    oh::set_num_prop(cx, &obj, "totalSessionTime", d.total_session_time as f64)?;
    oh::set_num_prop(
        cx,
        &obj,
        "total_session_distance",
        d.total_session_distance as f64,
    )?;
    oh::set_num_prop(cx, &obj, "x", d.x as f64)?;
    oh::set_num_prop(cx, &obj, "y", d.y as f64)?;
    oh::set_num_prop(cx, &obj, "z", d.z as f64)?;
    oh::set_num_prop(cx, &obj, "sessionTime", d.session_time as f64)?;
    oh::set_num_prop(cx, &obj, "sessionTimeLeft", d.session_time_left as f64)?;
    oh::set_num_prop(cx, &obj, "lapDistance", d.lap_distance as f64)?;
    oh::set_num_prop(cx, &obj, "totalDistance", d.total_distance as f64)?;
    oh::set_num_prop(cx, &obj, "totalLaps", d.total_laps as f64)?;
    oh::set_num_prop(cx, &obj, "carPosition", d.car_position as f64)?;
    oh::set_num_prop(cx, &obj, "inPits", d.in_pits as f64)?;
    oh::set_bool_prop(cx, &obj, "pitLimiterStatus", d.pit_limiter_status)?;
    oh::set_num_prop(cx, &obj, "pitSpeedLimit", d.pit_speed_limit as f64)?;
    oh::set_bool_prop(cx, &obj, "drs", d.drs)?;
    oh::set_num_prop(cx, &obj, "drsAllowed", d.drs_allowed as f64)?;
    oh::set_num_prop(cx, &obj, "vehicleFiaFlags", d.vehicle_fia_flags as f64)?;
    oh::set_num_prop(cx, &obj, "throttle", d.throttle as f64)?;
    oh::set_num_prop(cx, &obj, "steer", d.steer as f64)?;
    oh::set_num_prop(cx, &obj, "brake", d.brake as f64)?;
    oh::set_num_prop(cx, &obj, "gforceLat", d.gforce_lat as f64)?;
    oh::set_num_prop(cx, &obj, "gforceLon", d.gforce_lon as f64)?;
    oh::set_num_prop(cx, &obj, "gforceVert", d.gforce_vert as f64)?;
    oh::set_num_prop(cx, &obj, "engineRate", d.engine_rate as f64)?;
    oh::set_num_prop(cx, &obj, "revLightsPercent", d.rev_lights_percent as f64)?;
    oh::set_num_prop(cx, &obj, "maxRpm", d.max_rpm as f64)?;
    oh::set_num_prop(cx, &obj, "idleRpm", d.idle_rpm as f64)?;
    oh::set_num_prop(cx, &obj, "maxGears", d.max_gears as f64)?;
    oh::set_num_prop(cx, &obj, "tractionControl", d.traction_control as f64)?;
    oh::set_num_prop(cx, &obj, "antiLockBrakes", d.anti_lock_brakes as f64)?;
    oh::set_num_prop(cx, &obj, "frontBrakeBias", d.front_brake_bias as f64)?;
    oh::set_num_prop(cx, &obj, "fuelInTank", d.fuel_in_tank as f64)?;
    oh::set_num_prop(cx, &obj, "fuelCapacity", d.fuel_capacity as f64)?;
    oh::set_num_prop(cx, &obj, "fuelMix", d.fuel_mix as f64)?;
    oh::set_num_prop(cx, &obj, "engineTemperature", d.engine_temperature as f64)?;
    oh::set_num_prop(cx, &obj, "tyreCompound", d.tyre_compound as f64)?;
    oh::set_num_prop(
        cx,
        &obj,
        "frontLeftWingDamage",
        d.front_left_wing_damage as f64,
    )?;
    oh::set_num_prop(
        cx,
        &obj,
        "frontRightWingDamage",
        d.front_right_wing_damage as f64,
    )?;
    oh::set_num_prop(cx, &obj, "rearWingDamage", d.rear_wing_damage as f64)?;
    oh::set_num_prop(cx, &obj, "engineDamage", d.engine_damage as f64)?;
    oh::set_num_prop(cx, &obj, "gearBoxDamage", d.gear_box_damage as f64)?;
    oh::set_num_prop(cx, &obj, "exhaustDamage", d.exhaust_damage as f64)?;
    oh::set_num_prop(cx, &obj, "carsTotal", d.cars_total as f64)?;
    oh::set_num_prop(cx, &obj, "playerCarIndex", d.player_car_index as f64)?;

    let brakes_temperature = cx.empty_array();
    for i in 0..d.brakes_temperature.len() {
        ah::set_num_prop(
            cx,
            &brakes_temperature,
            i as u32,
            d.brakes_temperature[i] as f64,
        )?;
    }
    obj.set(cx, "brakesTemperature", brakes_temperature)?;

    let tyres_pressure = cx.empty_array();
    for i in 0..d.tyres_pressure.len() {
        ah::set_num_prop(cx, &tyres_pressure, i as u32, d.tyres_pressure[i] as f64)?;
    }
    obj.set(cx, "tyresPressure", tyres_pressure)?;

    let tyres_temperature = cx.empty_array();
    for i in 0..d.tyres_temperature.len() {
        ah::set_num_prop(
            cx,
            &tyres_temperature,
            i as u32,
            d.tyres_temperature[i] as f64,
        )?;
    }
    obj.set(cx, "tyresTemperature", tyres_temperature)?;

    let tyres_wear = cx.empty_array();
    for i in 0..d.tyres_wear.len() {
        ah::set_num_prop(cx, &tyres_wear, i as u32, d.tyres_wear[i] as f64)?;
    }
    obj.set(cx, "tyresWear", tyres_wear)?;

    let tyres_damage = cx.empty_array();
    for i in 0..d.tyres_damage.len() {
        ah::set_num_prop(cx, &tyres_damage, i as u32, d.tyres_damage[i] as f64)?;
    }
    obj.set(cx, "tyresDamage", tyres_damage)?;

    let car_data = cx.empty_array();
    for i in 0..d.car_data.len() {
        let car_data_obj = build_car_js_object(cx, &d.car_data[i]);
        ah::set_obj_prop(cx, &car_data, i as u32, car_data_obj)?;
    }
    obj.set(cx, "carData", car_data)?;

    Ok(obj)
}

fn build_car_js_object<'a>(
    cx: &mut FunctionContext<'a>,
    d: &Car,
) -> NeonResult<Handle<'a, JsObject>> {
    let obj = cx.empty_object();

    let world_position = cx.empty_array();
    for i in 0..d.world_position.len() {
        ah::set_num_prop(cx, &world_position, i as u32, d.world_position[i] as f64)?;
    }
    obj.set(cx, "worldPosition", world_position)?;

    oh::set_num_prop(cx, &obj, "lastLapTime", d.last_lap_time as f64)?;
    oh::set_num_prop(cx, &obj, "currentLapTime", d.current_lap_time as f64)?;
    oh::set_num_prop(cx, &obj, "bestLapTime", d.best_lap_time as f64)?;
    oh::set_num_prop(cx, &obj, "sector1Time", d.sector1_time as f64)?;
    oh::set_num_prop(cx, &obj, "sector2Time", d.sector2_time as f64)?;
    oh::set_num_prop(cx, &obj, "lapDistance", d.lap_distance as f64)?;
    oh::set_num_prop(cx, &obj, "driverId", d.driver_id as f64)?;
    oh::set_num_prop(cx, &obj, "teamId", d.team_id as f64)?;
    oh::set_num_prop(cx, &obj, "carPosition", d.car_position as f64)?;
    oh::set_num_prop(cx, &obj, "currentLapNum", d.current_lap_num as f64)?;
    oh::set_num_prop(cx, &obj, "inPits", d.in_pits as f64)?;
    oh::set_num_prop(cx, &obj, "sector", d.sector as f64)?;
    oh::set_num_prop(cx, &obj, "currentLapInvalid", d.current_lap_invalid as f64)?;
    oh::set_num_prop(cx, &obj, "penalties", d.penalties as f64)?;

    Ok(obj)
}

// #[allow(unused_must_use)]
// fn build_lap_js_object<'a>(scope: &mut scope::RootScope<'a>, lap: &Lap) -> Handle<'a, JsObject> {
//     let object = JsObject::new(scope);

//     obj.set(cx,
//         "sessionTimeStamp",
//         cx.number( lap.session_time_stamp as f64),
//     );

//     obj.set(cx, "lapNumber", cx.number( lap.lap_number as f64));
//     obj.set(cx, "lapTime", cx.number( lap.lap_time as f64));
//     obj.set(cx, "sector1Time", cx.number( lap.sector1_time as f64));
//     obj.set(cx, "sector2Time", cx.number( lap.sector2_time as f64));
//     obj.set(cx, "sector3Time", cx.number( lap.sector3_time as f64));

//     obj.set(cx,
//         "tyreCompound",
//         cx.number( lap.tyre_compound as f64),
//     );

//     obj.set(cx, "recordMarker", build_record_marker_js_object(scope, &lap.record_marker));

//     object
// }

// #[allow(unused_must_use)]
// fn build_sector_js_object<'a>(
//     scope: &mut scope::RootScope<'a>,
//     sector: &Sector,
// ) -> Handle<'a, JsObject> {
//     let object = JsObject::new(scope);

//     obj.set(cx,
//         "sessionTimeStamp",
//         cx.number( sector.session_time_stamp as f64),
//     );
//     obj.set(cx, "sector", cx.number( sector.sector as f64));
//     obj.set(cx,
//         "sectorTime",
//         cx.number( sector.sector_time as f64),
//     );
//     obj.set(cx,
//         "tyreCompound",
//         cx.number( sector.tyre_compound as f64),
//     );

//     obj.set(cx, "recordsMarker", build_record_marker_js_object(scope, &sector.record_marker));

//     object
// }

// #[allow(unused_must_use)]
// fn build_record_marker_js_object<'a>(
//     scope: &mut scope::RootScope<'a>,
//     record_marker: &RecordMarker,
// ) -> Handle<'a, JsObject> {
//     let object = JsObject::new(scope);

//     obj.set(cx, "isBestEverPersonal", JsBoolean::new(scope, record_marker.is_best_ever_personal));
//     obj.set(cx, "isBestEverCompoundPersonal", JsBoolean::new(scope, record_marker.is_best_ever_compound_personal));
//     obj.set(cx, "isBestSessionPersonal", JsBoolean::new(scope, record_marker.is_best_session_personal));
//     obj.set(cx,
//         "isBestSessionPersonalCompound",
//         JsBoolean::new(scope, record_marker.is_best_session_personal_compound),
//     );
//     obj.set(cx, "isBestSessionAll", JsBoolean::new(scope, record_marker.is_best_session_all));
//     obj.set(cx, "isBestSessionAllCompound", JsBoolean::new(scope, record_marker.is_best_session_all_compound));

//     object
// }

// register_module!(m, {
//     m.export("initialise", initialise)
//         .expect("failed to export initialise");
//     m.export("getNextTick", get_next_tick)
//         .expect("failed to export getNextTick");
//     m.export("startListening", start_listening)
//         .expect("failed to export startListening");
//     m.export("replayAllLaps", replay_all_laps)
//         .expect("failed to export replayAllLaps");
//     m.export("getLapData", get_lap_data)
//         .expect("failed to export getLapData");
//     m.export("getAllLapsMetadata", get_all_laps_metadata)
//         .expect("failed to export getAllLapsMetadata");
//     m.export("replayLap", replay_lap)
//         .expect("failed to export replayLap");
//     Ok(())
// });

register_module!(mut cx, {
    cx.export_function("initialise", initialise)?;
    cx.export_function("startListening", start_listening)?;
    cx.export_function("replayAllLaps", replay_all_laps)?;
    cx.export_function("getNextTick", get_next_tick)?;
    Ok(())
});
