#![allow(unknown_lints)]
#![deny(clippy)]

#[macro_use]
extern crate neon;
extern crate neon_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate f1_laps_core;

use f1_laps_core::prelude::*;
use neon::context::Context;
use neon::prelude::*;
use serde::ser::Serialize;
use std::sync::Mutex;

lazy_static! {
    static ref COLLECTOR: Mutex<Collector> = Mutex::new(Default::default());
    static ref LOGS: Mutex<Vec<LogItem>> = Mutex::new(Vec::new());
}

#[derive(Clone, Serialize)]
pub struct LogItem {
    pub event: LogEvent,
    pub message: String,
}

#[derive(Default)]
pub struct Collector {
    context: Option<&'static AppContext>,
    session_identifier: Option<SessionIdentifier>,
    finished_lap: Option<Lap>,
    finished_sector: Option<Sector>,
    session_data: Option<SessionData>,
    lap_data: Option<OptMultiCarData<LapData>>,
    car_status: Option<OptMultiCarData<CarStatus>>,
    car_telemetry: Option<OptMultiCarData<CarTelemetry>>,
    car_motion: Option<OptMultiCarData<CarMotion>>,
    car_setup: Option<OptMultiCarData<CarSetup>>,
    participants_info: Option<OptMultiCarData<ParticipantInfo>>,
}

impl Collector {
    pub fn update(&mut self, output: Output) {
        if output.events.started_session.is_some() {
            self.session_identifier = output.events.started_session;
        }

        if output.events.finished_lap.is_some() {
            self.finished_lap = output.events.finished_lap;
        }

        if output.events.finished_sector.is_some() {
            self.finished_sector = output.events.finished_sector;
        }

        if output.session_data.is_some() {
            self.session_data = output.session_data;
        }

        self.lap_data = Some(output.lap_data);

        if output.car_status.is_some() {
            self.car_status = output.car_status;
        }

        self.car_telemetry = Some(output.car_telemetry);

        self.car_motion = Some(output.car_motion);

        if output.car_setup.is_some() {
            self.car_setup = output.car_setup;
        }

        if output.participants_info.is_some() {
            self.participants_info = output.participants_info;
        }
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

    pub fn get_car_setup(&mut self) -> Option<OptMultiCarData<CarSetup>> {
        let res = self.car_setup.clone();
        self.car_setup = None;
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

    let mut collector = COLLECTOR.lock().unwrap();

    let context = f1_laps_core::initialise(&storage_folder_path, on_log_received);
    collector.context = Some(Box::leak(context));

    Ok(JsUndefined::new())
}

fn start_listening(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let port = cx.argument::<JsNumber>(0)?.value() as i32;
    let should_store_packets = cx.argument::<JsBoolean>(1)?.value();

    let collector = COLLECTOR.lock().unwrap();
    let x = collector.context.unwrap();

    f1_laps_core::start_listening(x, port, should_store_packets, on_output_received);

    Ok(JsUndefined::new())
}

fn replay_packets(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let should_simulate_time = cx.argument::<JsBoolean>(0)?.value();

    let collector = COLLECTOR.lock().unwrap();

    f1_laps_core::replay_packets(
        &collector.context.unwrap(),
        should_simulate_time,
        false,
        on_output_received,
    );

    Ok(JsUndefined::new())
}

fn get_laps(mut cx: FunctionContext) -> JsResult<JsValue> {
    let collector = COLLECTOR.lock().unwrap();
    let laps = f1_laps_core::get_laps_headers(&collector.context.unwrap());

    let js_array = neon_serde::to_value(&mut cx, &laps)?;
    Ok(js_array)
}

fn get_lap_telemetry(mut cx: FunctionContext) -> JsResult<JsValue> {
    let lap_id = cx.argument::<JsString>(0)?.value();

    let collector = COLLECTOR.lock().unwrap();
    let telemetry = f1_laps_core::get_lap_telemetry(&collector.context.unwrap(), &lap_id);

    let js_array = neon_serde::to_value(&mut cx, &telemetry)?;
    Ok(js_array)
}

fn delete_lap(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let lap_id = cx.argument::<JsString>(0)?.value();

    let collector = COLLECTOR.lock().unwrap();
    f1_laps_core::delete_lap(&collector.context.unwrap(), &lap_id);

    Ok(JsUndefined::new())
}

fn get_next_tick(mut cx: FunctionContext) -> JsResult<JsObject> {
    let object = cx.empty_object();

    let mut logs = LOGS.lock().unwrap();
    if logs.len() > 0 {
        append_as_js(&mut cx, "logs", Some(logs.to_vec()).as_ref(), object)?;
        logs.clear();
    }

    let mut collector = COLLECTOR.lock().unwrap();
    append_as_js(
        &mut cx,
        "sessionIdentifier",
        collector.get_session_identifier().as_ref(),
        object,
    )?;

    append_as_js(
        &mut cx,
        "finishedLap",
        collector.get_finished_lap().as_ref(),
        object,
    )?;

    append_as_js(
        &mut cx,
        "finishedSector",
        collector.get_finished_sector().as_ref(),
        object,
    )?;

    append_as_js(
        &mut cx,
        "sessionData",
        collector.get_session_data().as_ref(),
        object,
    )?;

    append_as_js(
        &mut cx,
        "lapData",
        collector.get_lap_data().as_ref(),
        object,
    )?;

    append_as_js(
        &mut cx,
        "carStatus",
        collector.get_car_status().as_ref(),
        object,
    )?;

    append_as_js(
        &mut cx,
        "carTelemetry",
        collector.get_car_telemetry().as_ref(),
        object,
    )?;

    append_as_js(
        &mut cx,
        "carMotion",
        collector.get_car_motion().as_ref(),
        object,
    )?;

    append_as_js(
        &mut cx,
        "carSetup",
        collector.get_car_setup().as_ref(),
        object,
    )?;

    append_as_js(
        &mut cx,
        "participants",
        collector.get_participants_info().as_ref(),
        object,
    )?;

    Ok(object)
}

fn on_output_received(output: Output) {
    let mut collector = COLLECTOR.lock().unwrap();
    collector.update(output);
}

fn on_log_received(e: LogEvent, m: &str) {
    let mut logs = LOGS.lock().unwrap();
    logs.push(LogItem {
        event: e,
        message: m.to_string(),
    });
}

fn append_as_js<'j, C, V>(
    cx: &mut C,
    key: &str,
    option: Option<&V>,
    object: Handle<'j, JsObject>,
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
    cx.export_function("getLaps", get_laps)?;
    cx.export_function("getLapTelemetry", get_lap_telemetry)?;
    cx.export_function("deleteLap", delete_lap)?;
    Ok(())
});
