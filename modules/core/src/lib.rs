#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate bincode;
extern crate chrono;
extern crate schedule_recv;

mod file_system;
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

pub fn initialise(storage_folder_path: String) {
    storage::initialise(&storage_folder_path);
}

pub fn start_listening<F>(
    port: i32,
    should_store_packets: bool,
    f: F,
) -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>)
where
    F: Fn(Output) + Send + Sync + 'static,
{
    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    let t = thread::spawn(move || {
        udp::start_listening(port, should_store_packets, tx);
    });

    let mut pipeline = Pipeline::new();

    let r = thread::spawn(move || loop {
        if let Some(tick) = rx.recv().ok() {
            let output = pipeline.process(tick);
            f(output);
        }
    });

    (t, r)
}

pub fn replay_packets<F>(
    should_simulate_time: bool,
    f: F,
) -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>)
where
    F: Fn(Output) + Send + Sync + 'static,
{
    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    let mut pipeline = Pipeline::new();

    let t = thread::spawn(move || {
        udp::replay_packets(should_simulate_time, tx);
    });

    let r = thread::spawn(move || loop {
        match rx.try_recv() {
            Ok(tick) => {
                let output = pipeline.process(tick);
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
