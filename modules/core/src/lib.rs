#![allow(unknown_lints)]
#![warn(clippy)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate chrono;
extern crate schedule_recv;
extern crate sled;
extern crate uuid;

mod pipeline;
pub mod prelude;
mod serialisation;
mod storage;
mod udp;

use pipeline::input::Tick;
use pipeline::output::Output;
use pipeline::Pipeline;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use storage::models::{LapHeader, LapTelemetry};
use storage::Storage;

pub use pipeline::output::*;

pub struct Context {
    pub(crate) storage: Storage,
}

pub fn initialise(storage_folder_path: &str) -> Box<Context> {
    let context = Context {
        storage: Storage::new(&storage_folder_path),
    };

    Box::new(context)
}

pub fn start_listening<F>(
    context: &'static Context,
    port: i32,
    should_store_packets: bool,
    f: F,
) -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>)
where
    F: Fn(Output) + Send + Sync + 'static,
{
    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    let t = thread::spawn(move || {
        udp::start_listening(&context.storage, port, should_store_packets, &tx);
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
    context: &'static Context,
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
        udp::replay_packets(&context.storage, should_simulate_time, tx);
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

pub fn get_laps_headers(context: &'static Context) -> Vec<LapHeader> {
    context.storage.lap_headers.get_all()
}

pub fn get_lap_telemetry(context: &'static Context, lap_id: &str) -> Option<LapTelemetry> {
    context.storage.lap_telemetry.get(lap_id)
}

pub fn delete_lap(context: &'static Context, lap_id: &str) {
    context.storage.lap_headers.delete(lap_id);
    context.storage.lap_telemetry.delete(lap_id);
}