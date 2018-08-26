#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate bincode;
extern crate chrono;

mod file_system;
mod lap_metadata;
mod pipeline;
pub mod prelude;
mod record_tracking;
mod replay;
mod serialisation;
mod storage;
mod udp;

use lap_metadata::LapMetadata;
use pipeline::types::*;
use pipeline::Pipeline;
use record_tracking::RecordSet;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

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
            let output = pipeline.process(&tick);
            f(output);
        }
    });

    (t, r)
}

// pub fn replay_packets<F>(
//     shoud_simulate_time: bool,
//     f: F,
// ) -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>)
// where
//     F: Fn(Output) + Send + Sync + 'static,
// {
//     let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

//     let t = thread::spawn(move || {
//         let packets = storage::get_all_packets();
//         replay::stream(tx, packets, shoud_simulate_time);
//     });

//     let mut pipeline = Pipeline::new();
//     pipeline.set_should_wait_for_fs(false);
//     pipeline.set_should_store_laps(false);

//     let r = thread::spawn(move || loop {
//         match rx.try_recv() {
//             Ok(tick) => {
//                 let output = pipeline.process(&tick);
//                 f(output);
//             }
//             Err(TryRecvError::Disconnected) => {
//                 break;
//             }
//             Err(TryRecvError::Empty) => {}
//         }
//     });

//     (t, r)
// }

pub fn replay_lap<F>(
    identifier: String,
    shoud_simulate_time: bool,
    f: F,
) -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>)
where
    F: Fn(Output) + Send + Sync + 'static,
{
    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    let t = thread::spawn(move || match storage::get_lap_data(&identifier) {
        Some(ticks) => replay::stream(tx, ticks, shoud_simulate_time),
        None => println!("no lap data found for identifier: {}", identifier), // TODO: add some sort of messaging/feedback mechanism
    });

    let mut pipeline = Pipeline::new();
    pipeline.set_should_wait_for_fs(false);
    pipeline.set_should_store_laps(false);

    let r = thread::spawn(move || loop {
        if let Some(tick) = rx.recv().ok() {
            let output = pipeline.process(&tick);
            f(output);
        }
    });

    (t, r)
}

pub fn get_all_laps_metadata() -> Vec<LapMetadata> {
    return storage::get_all_laps_metadata();
}

pub fn get_all_records() -> RecordSet {
    return storage::get_all_records();
}

#[cfg(test)]
pub(crate) mod test_utils {
    use pipeline::types::Tick;

    pub fn create_tick() -> Tick {
        Tick {
            session_time: 123 as f32,
            session_distance: 123 as f32,
            lap_time: 123 as f32,
            lap_distance: 123 as f32,
            x: 1 as f32,
            y: 2 as f32,
            z: 3 as f32,
            speed: 123 as f32,
            throttle: 12 as f32,
            steer: 12 as f32,
            brake: 12 as f32,
            gear: 3,
            lap_number: 1,
            rev_lights_percent: 0,
            tyres_wear: [0; 4],
            car_position: 2,
            is_drs_open: false,
            sector_number: 1,
            sector1_time: 0 as f32,
            sector2_time: 0 as f32,
            team_id: 1,
            total_laps: 0,
            last_lap_time: 0 as f32,
            max_gears: 8,
            session_type: 1,
            track_id: 1,
            vehicle_fia_flags: -1,
            era: 2017,
            tyre_compound: 2,
            is_current_lap_valid: true,
            is_spectating: false,
            car_index: 0,
            cars_total: 0,
            cars: Vec::new(),
        }
    }
}
