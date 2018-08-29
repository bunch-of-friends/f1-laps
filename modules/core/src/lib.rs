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

use std::sync::mpsc::{self, TryRecvError};
use std::thread;

use pipeline::input::Tick;
use pipeline::output::Output;
use pipeline::Pipeline;
use serialisation::ReceivePacket;

pub fn initialise(storage_folder_path: String) {
    storage::initialise(&storage_folder_path);
}

pub fn start_listening<F>(
    port: i32,
    f: F,
) -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>)
where
    F: Fn(Output) + Send + Sync + 'static,
{
    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    let t = thread::spawn(move || {
        udp::start_listening(port, serialisation::get_serialiser(), tx);
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

pub fn replay_packets<F>(f: F) -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>)
where
    F: Fn(Output) + Send + Sync + 'static,
{
    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    let mut pipeline = Pipeline::new();

    let t = thread::spawn(move || {
        let packets = storage::get_all_packets();
        let serialiser = serialisation::get_serialiser();
        for packet in &packets {
            if let Some(tick) = serialiser.converto_to_tick(packet, packet.len()) {
                tx.send(tick).ok();
            }
        }
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
