#[macro_use]
extern crate neon;
extern crate neon_serde;
#[macro_use]
extern crate lazy_static;
extern crate f1_laps_core;
extern crate serde;

use f1_laps_core::prelude::*;
use neon::context::Context;
use neon::prelude::*;
use serde::ser::Serialize;
use std::sync::Mutex;

lazy_static! {
    static ref COLLECTOR: Mutex<Collector> = Mutex::new(Collector::new());
}

pub struct Collector {
    session_identifier: Option<SessionIdentifier>,
    finished_lap: Option<Lap>,
    finished_sector: Option<Sector>,
    session_data: Option<SessionData>,
    lap_data: Option<OptMultiCarData<LapData>>,
    car_status: Option<OptMultiCarData<CarStatus>>,
    car_telemetry: Option<OptMultiCarData<CarTelemetry>>,
    car_motion: Option<OptMultiCarData<CarMotion>>,
    participants_info: Option<OptMultiCarData<ParticipantInfo>>,
}

impl Collector {
    pub fn new() -> Collector {
        Collector {
            session_identifier: None,
            finished_lap: None,
            finished_sector: None,
            session_data: None,
            lap_data: None,
            car_status: None,
            car_telemetry: None,
            car_motion: None,
            participants_info: None,
        }
    }

    pub fn update(&mut self, output: &Output) {
        if output.events.started_session.is_some() {
            self.session_identifier = output.events.started_session.clone();
        }

        if output.events.finished_lap.is_some() {
            self.finished_lap = output.events.finished_lap.clone();
        }

        if output.events.finished_sector.is_some() {
            self.finished_sector = output.events.finished_sector.clone();
        }

        if output.session_data.is_some() {
            self.session_data = output.session_data.clone();
        }

        self.lap_data = Some(output.lap_data.clone());

        if output.car_status.is_some() {
            self.car_status = output.car_status.clone();
        }

        self.car_telemetry = Some(output.car_telemetry.clone());

        self.car_motion = Some(output.car_motion.clone());
    }

    pub fn get_session_identifier(&mut self) -> Option<SessionIdentifier> {
        let res = self.session_identifier.clone();
        self.session_identifier = None;
        res
    }

    pub fn get_finished_lap(&mut self) -> Option<Lap> {
        let res = self.finished_lap.clone();
        self.finished_lap = None;
        res
    }

    pub fn get_finished_sector(&mut self) -> Option<Sector> {
        let res = self.finished_sector.clone();
        self.finished_sector = None;
        res
    }

    pub fn get_session_data(&mut self) -> Option<SessionData> {
        let res = self.session_data.clone();
        self.session_data = None;
        res
    }

    pub fn get_lap_data(&mut self) -> Option<OptMultiCarData<LapData>> {
        let res = self.lap_data.clone();
        self.lap_data = None;
        res
    }

    pub fn get_car_status(&mut self) -> Option<OptMultiCarData<CarStatus>> {
        let res = self.car_status.clone();
        self.car_status = None;
        res
    }

    pub fn get_car_telemetry(&mut self) -> Option<OptMultiCarData<CarTelemetry>> {
        let res = self.car_telemetry.clone();
        self.car_telemetry = None;
        res
    }

    pub fn get_car_motion(&mut self) -> Option<OptMultiCarData<CarMotion>> {
        let res = self.car_motion.clone();
        self.car_motion = None;
        res
    }

    pub fn get_participants_info(&mut self) -> Option<OptMultiCarData<ParticipantInfo>> {
        let res = self.participants_info.clone();
        self.participants_info = None;
        res
    }
}

fn initialise(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let storage_folder_path = cx.argument::<JsString>(0)?.value();

    f1_laps_core::initialise(storage_folder_path);

    Ok(JsUndefined::new())
}

fn start_listening(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let port = cx.argument::<JsNumber>(0)?.value() as i32;
    let should_store_packets = cx.argument::<JsBoolean>(1)?.value();

    f1_laps_core::start_listening(port, should_store_packets, on_output_received);

    Ok(JsUndefined::new())
}

fn replay_packets(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let should_simulate_time = cx.argument::<JsBoolean>(0)?.value();

    f1_laps_core::replay_packets(should_simulate_time, on_output_received);

    Ok(JsUndefined::new())
}

fn on_output_received(output: Output) {
    let mut collector = COLLECTOR.lock().unwrap();
    collector.update(&output);
}

fn get_next_tick(mut cx: FunctionContext) -> JsResult<JsObject> {
    let mut collector = COLLECTOR.lock().unwrap();

    let object = cx.empty_object();

    append_as_js(
        &mut cx,
        "sessionIdentifier",
        collector.get_session_identifier().as_ref(),
        &object,
    )?;

    append_as_js(
        &mut cx,
        "finishedLap",
        collector.get_finished_lap().as_ref(),
        &object,
    )?;

    append_as_js(
        &mut cx,
        "finishedSector",
        collector.get_finished_sector().as_ref(),
        &object,
    )?;

    append_as_js(
        &mut cx,
        "sessionData",
        collector.get_session_data().as_ref(),
        &object,
    )?;

    append_as_js(
        &mut cx,
        "lapData",
        collector.get_lap_data().as_ref(),
        &object,
    )?;

    append_as_js(
        &mut cx,
        "carStatus",
        collector.get_car_status().as_ref(),
        &object,
    )?;

    append_as_js(
        &mut cx,
        "carTelemetry",
        collector.get_car_telemetry().as_ref(),
        &object,
    )?;

    append_as_js(
        &mut cx,
        "carMotion",
        collector.get_car_motion().as_ref(),
        &object,
    )?;

    append_as_js(
        &mut cx,
        "participants",
        collector.get_participants_info().as_ref(),
        &object,
    )?;

    Ok(object)
}

fn append_as_js<'j, C, V>(
    cx: &mut C,
    key: &str,
    option: Option<&V>,
    object: &Handle<'j, JsObject>,
) -> NeonResult<bool>
where
    C: Context<'j>,
    V: Serialize + ?Sized,
{
    if let Some(value) = option {
        let js_value = neon_serde::to_value(cx, value)?;
        object.set(cx, key, js_value)
    } else {
        Ok(false)
    }
}

register_module!(mut cx, {
    cx.export_function("initialise", initialise)?;
    cx.export_function("startListening", start_listening)?;
    cx.export_function("replayPackets", replay_packets)?;
    cx.export_function("getNextTick", get_next_tick)?;
    Ok(())
});
