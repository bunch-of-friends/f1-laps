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

    let live_data_obj = build_live_data_js_object(&mut cx, &tick.live_data);
    oh::set_obj_prop(&mut cx, &object, "liveData", live_data_obj)?;

    if let Some(session) = tick.session_started {
        let session_obj = build_session_js_object(&mut cx, &session);
        oh::set_obj_prop(&mut cx, &object, "sessionStarted", session_obj)?;
    }

    if let Some(lap) = tick.lap_finished {
        let lap_obj = build_lap_js_object(&mut cx, &lap);
        oh::set_obj_prop(&mut cx, &object, "lapFinished", lap_obj)?;
    }

    if let Some(sector) = tick.sector_finished {
        let sector_obj = build_sector_js_object(&mut cx, &sector);
        oh::set_obj_prop(&mut cx, &object, "sectorFinished", sector_obj)?;
    }

    Ok(object)
}

fn get_lap_data(mut cx: FunctionContext) -> JsResult<JsArray> {
    let identifier = cx.argument::<JsString>(0)?.value();

    let data = f1_laps_core::get_lap_data(identifier);
    let arr = cx.empty_array();

    let mut index = 0;
    for item in data.iter() {
        let js_object = build_live_data_js_object(&mut cx, item);
        ah::set_obj_item(&mut cx, &arr, index, js_object)?;
        index += 1;
    }

    Ok(arr)
}

fn get_all_laps_metadata(mut cx: FunctionContext) -> JsResult<JsArray> {
    let metadata = f1_laps_core::get_all_laps_metadata();
    let arr = cx.empty_array();
    let mut index = 0;
    for item in metadata.iter() {
        let js_object = build_lap_metadata_js_object(&mut cx, &item);
        ah::set_obj_item(&mut cx, &arr, index, js_object)?;
        index += 1;
    }

    Ok(arr)
}

fn replay_lap(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let identifier = cx.argument::<JsString>(0)?.value();

    f1_laps_core::replay_lap(identifier);

    Ok(JsUndefined::new())
}

fn build_lap_metadata_js_object<'a>(
    cx: &mut FunctionContext<'a>,
    d: &LapMetadata,
) -> NeonResult<Handle<'a, JsObject>> {
    let obj = cx.empty_object();

    oh::set_str_prop(cx, &obj, "identifier", d.identifier.as_str())?;
    oh::set_str_prop(cx, &obj, "recordedDate", d.recorded_date.as_str())?;
    oh::set_num_prop(cx, &obj, "trackId", d.track_id as f64)?;
    oh::set_num_prop(cx, &obj, "teamId", d.team_id as f64)?;
    oh::set_num_prop(cx, &obj, "era", d.era as f64)?;
    oh::set_num_prop(cx, &obj, "tyreCompound", d.tyre_compound as f64)?;
    oh::set_num_prop(cx, &obj, "sessionType", d.session_type as f64)?;
    oh::set_num_prop(cx, &obj, "lapNumber", d.lap_number as f64)?;
    oh::set_num_prop(cx, &obj, "lapTime", d.lap_time as f64)?;
    oh::set_str_prop(cx, &obj, "note", d.note.as_str())?;

    let sector_times = cx.empty_array();
    for i in 0..d.sector_times.len() {
        ah::set_num_item(cx, &sector_times, i as u32, d.sector_times[i] as f64)?;
    }
    obj.set(cx, "sectorTimes", sector_times)?;

    Ok(obj)
}

fn build_session_js_object<'a>(
    cx: &mut FunctionContext<'a>,
    d: &Session,
) -> NeonResult<Handle<'a, JsObject>> {
    let obj = cx.empty_object();

    oh::set_num_prop(cx, &obj, "sessionTimeStamp", d.session_time_stamp as f64)?;
    oh::set_num_prop(cx, &obj, "era", d.era as f64)?;
    oh::set_num_prop(cx, &obj, "trackId", d.track_id as f64)?;
    oh::set_num_prop(cx, &obj, "teamId", d.team_id as f64)?;
    oh::set_num_prop(cx, &obj, "sessionType", d.session_type as f64)?;

    Ok(obj)
}

fn build_live_data_js_object<'a>(
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
        ah::set_num_item(
            cx,
            &brakes_temperature,
            i as u32,
            d.brakes_temperature[i] as f64,
        )?;
    }
    obj.set(cx, "brakesTemperature", brakes_temperature)?;

    let tyres_pressure = cx.empty_array();
    for i in 0..d.tyres_pressure.len() {
        ah::set_num_item(cx, &tyres_pressure, i as u32, d.tyres_pressure[i] as f64)?;
    }
    obj.set(cx, "tyresPressure", tyres_pressure)?;

    let tyres_temperature = cx.empty_array();
    for i in 0..d.tyres_temperature.len() {
        ah::set_num_item(
            cx,
            &tyres_temperature,
            i as u32,
            d.tyres_temperature[i] as f64,
        )?;
    }
    obj.set(cx, "tyresTemperature", tyres_temperature)?;

    let tyres_wear = cx.empty_array();
    for i in 0..d.tyres_wear.len() {
        ah::set_num_item(cx, &tyres_wear, i as u32, d.tyres_wear[i] as f64)?;
    }
    obj.set(cx, "tyresWear", tyres_wear)?;

    let tyres_damage = cx.empty_array();
    for i in 0..d.tyres_damage.len() {
        ah::set_num_item(cx, &tyres_damage, i as u32, d.tyres_damage[i] as f64)?;
    }
    obj.set(cx, "tyresDamage", tyres_damage)?;

    let car_data = cx.empty_array();
    for i in 0..d.car_data.len() {
        let car_data_obj = build_car_js_object(cx, &d.car_data[i]);
        ah::set_obj_item(cx, &car_data, i as u32, car_data_obj)?;
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
        ah::set_num_item(cx, &world_position, i as u32, d.world_position[i] as f64)?;
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

fn build_lap_js_object<'a>(
    cx: &mut FunctionContext<'a>,
    d: &Lap,
) -> NeonResult<Handle<'a, JsObject>> {
    let obj = cx.empty_object();

    oh::set_num_prop(cx, &obj, "sessionTimeStamp", d.session_time_stamp as f64)?;
    oh::set_num_prop(cx, &obj, "lapNumber", d.lap_number as f64)?;
    oh::set_num_prop(cx, &obj, "lapTime", d.lap_time as f64)?;
    oh::set_num_prop(cx, &obj, "sector1Time", d.sector1_time as f64)?;
    oh::set_num_prop(cx, &obj, "sector2Time", d.sector2_time as f64)?;
    oh::set_num_prop(cx, &obj, "sector3Time", d.sector3_time as f64)?;
    oh::set_num_prop(cx, &obj, "tyreCompound", d.tyre_compound as f64)?;

    let record_marker_obj = build_record_marker_js_object(cx, &d.record_marker);
    oh::set_obj_prop(cx, &obj, "recordMarker", record_marker_obj)?;

    Ok(obj)
}

fn build_sector_js_object<'a>(
    cx: &mut FunctionContext<'a>,
    d: &Sector,
) -> NeonResult<Handle<'a, JsObject>> {
    let obj = cx.empty_object();

    oh::set_num_prop(cx, &obj, "sessionTimeStamp", d.session_time_stamp as f64)?;
    oh::set_num_prop(cx, &obj, "sector", d.sector as f64)?;
    oh::set_num_prop(cx, &obj, "sectorTime", d.sector_time as f64)?;
    oh::set_num_prop(cx, &obj, "tyreCompound", d.tyre_compound as f64)?;

    let record_marker_obj = build_record_marker_js_object(cx, &d.record_marker);
    oh::set_obj_prop(cx, &obj, "recordMarker", record_marker_obj)?;

    Ok(obj)
}

fn build_record_marker_js_object<'a>(
    cx: &mut FunctionContext<'a>,
    d: &RecordMarker,
) -> NeonResult<Handle<'a, JsObject>> {
    let obj = cx.empty_object();

    oh::set_bool_prop(cx, &obj, "isBestEverPersonal", d.is_best_ever_personal)?;
    oh::set_bool_prop(
        cx,
        &obj,
        "isBestEverCompoundPersonal",
        d.is_best_ever_compound_personal,
    )?;
    oh::set_bool_prop(
        cx,
        &obj,
        "isBestSessionPersonal",
        d.is_best_session_personal,
    )?;
    oh::set_bool_prop(
        cx,
        &obj,
        "isBestSessionPersonalCompound",
        d.is_best_session_personal_compound,
    )?;
    oh::set_bool_prop(cx, &obj, "isBestSessionAll", d.is_best_session_all)?;
    oh::set_bool_prop(
        cx,
        &obj,
        "isBestSessionAllCompound",
        d.is_best_session_all_compound,
    )?;

    Ok(obj)
}

register_module!(mut cx, {
    cx.export_function("initialise", initialise)?;
    cx.export_function("startListening", start_listening)?;
    cx.export_function("replayAllLaps", replay_all_laps)?;
    cx.export_function("getNextTick", get_next_tick)?;
    cx.export_function("getLapData", get_lap_data)?;
    cx.export_function("getAllLapsMetadata", get_all_laps_metadata)?;
    cx.export_function("replayLap", replay_lap)?;
    Ok(())
});
