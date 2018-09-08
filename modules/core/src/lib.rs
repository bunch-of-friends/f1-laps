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
use storage::Storage;

pub use pipeline::output::*;

pub struct Context {
    pub(crate) storage: Storage,
}

pub fn initialise(storage_folder_path: String) -> Box<Context> {
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
        udp::start_listening(&context.storage, port, should_store_packets, tx);
    });

    let mut pipeline = Pipeline::new(true);

    let r = thread::spawn(move || loop {
        if let Some(tick) = rx.recv().ok() {
            let output = pipeline.process(&context.storage, tick);
            f(output);
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
