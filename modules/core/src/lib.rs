#![allow(unknown_lints)]
#![deny(clippy)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate chrono;
extern crate schedule_recv;
extern crate sled;
extern crate uuid;

mod context;
mod pipeline;
pub mod prelude;
mod serialisation;
mod storage;
mod udp;

use context::{AppContext, LogEvent};
use pipeline::input::Tick;
use pipeline::output::Output;
use pipeline::Pipeline;
use std::cmp::Ordering;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use storage::models::{LapHeader, LapTelemetry};
use storage::Storage;

pub use pipeline::output::*;

pub fn initialise<F>(storage_folder_path: &str, logger: F) -> Box<AppContext>
where
    F: Fn(LogEvent, &str) + Send + Sync + 'static,
{
    let context = AppContext {
        storage: Storage::new(&storage_folder_path),
        logger: Box::new(logger),
    };

    context.log(LogEvent::UserInfo, "Initialised");

    Box::new(context)
}

pub fn start_listening<F>(
    context: &'static AppContext,
    port: i32,
    should_store_packets: bool,
    f: F,
) -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>)
where
    F: Fn(Output) + Send + Sync + 'static,
{
    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    let t = thread::spawn(move || {
        udp::start_listening(&context, port, should_store_packets, &tx);
    });

    let mut pipeline = Pipeline::new(true);

    let r = thread::spawn(move || loop {
        match rx.recv() {
            Ok(tick) => {
                let output = pipeline.process(&context.storage, tick);
                f(output);
            }
            Err(_) => print!("error receiving tick"),
        }
    });

    (t, r)
}

pub fn replay_packets<F>(
    context: &'static AppContext,
    should_simulate_time: bool,
    should_store_laps: bool,
    f: F,
) -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>)
where
    F: Fn(Output) + Send + Sync + 'static,
{
    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    let mut pipeline = Pipeline::new(should_store_laps);

    let t = thread::spawn(move || {
        udp::replay_packets(&context, should_simulate_time, tx);
    });

    let r = thread::spawn(move || loop {
        match rx.try_recv() {
            Ok(tick) => {
                let output = pipeline.process(&context.storage, tick);
                f(output);
            }
            Err(TryRecvError::Disconnected) => {
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    });

    (t, r)
}

pub fn get_laps_headers(context: &'static AppContext) -> Vec<LapHeader> {
    let mut laps = context.storage.lap_headers.get_all();
    laps.sort_by(|a, b| {
        if a.recorded_date > b.recorded_date {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
    laps
}

pub fn get_lap_telemetry(context: &'static AppContext, lap_id: &str) -> Option<LapTelemetry> {
    context.storage.lap_telemetry.get(lap_id)
}

pub fn delete_lap(context: &'static AppContext, lap_id: &str) {
    context.storage.lap_headers.delete(lap_id);
    context.storage.lap_telemetry.delete(lap_id);
}
