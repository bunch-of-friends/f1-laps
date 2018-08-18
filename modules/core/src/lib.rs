#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate bincode;
extern crate chrono;

pub mod aggregation;
pub mod file_system;
pub mod lap_metadata;
pub mod prelude;
pub mod record_tracking;
pub mod replay;
pub mod storage;
pub mod udp;

use aggregation::collector::Collector;
use aggregation::tick::{LiveData, Tick};
use lap_metadata::LapMetadata;
use record_tracking::RecordSet;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::Mutex;
use std::thread;

lazy_static! {
    static ref DATA_HOLDER: Mutex<Collector> = Mutex::new(Collector::new());
}

pub fn initialise(storage_folder_path: String) {
    storage::initialise(&storage_folder_path);
}

pub fn start_listening(port: i32) {
    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    thread::spawn(move || {
        udp::start_listening(port, tx);
    });

    thread::spawn(move || loop {
        receive_tick(&rx);
    });
}

pub fn get_next_tick() -> Option<Tick> {
    let mut data_holder = DATA_HOLDER.lock().unwrap();

    let data = data_holder.get_data();

    if let Some(live_data) = data.0 {
        Some(Tick {
            live_data: live_data,
            session_started: data.1,
            lap_finished: data.2,
            sector_finished: data.3,
            message: None,
        })
    } else {
        None
    }
}

pub fn get_all_laps_metadata() -> Vec<LapMetadata> {
    return storage::get_all_laps_metadata();
}

pub fn get_all_records() -> RecordSet {
    return storage::get_all_records();
}

pub fn get_lap_data(identifier: String) -> Vec<LiveData> {
    let packets = match storage::get_lap_data(&identifier) {
        Some(x) => x,
        None => panic!("no lap data found for identifier: {}", identifier), // TODO: add some sort of messaging/feedback mechanism
    };

    return aggregation::convert_packets(&packets);
}

pub fn replay_lap(identifier: String) {
    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    thread::spawn(move || match storage::get_lap_data(&identifier) {
        Some(packets) => replay::stream_packets(tx, packets),
        None => println!("no lap data found for identifier: {}", identifier), // TODO: add some sort of messaging/feedback mechanism
    });

    thread::spawn(move || loop {
        receive_tick(&rx);
    });
}

pub fn replay_all_laps() {
    let (tx, rx): (mpsc::Sender<Tick>, mpsc::Receiver<Tick>) = mpsc::channel();

    thread::spawn(move || {
        let packets = storage::get_all_laps_data();
        replay::stream_packets(tx, packets);
    });

    thread::spawn(move || loop {
        receive_tick(&rx);
    });
}

// new/refactor
pub mod conversion;
pub mod pipeline;

use pipeline::types::*;

pub fn replay_all_laps_new() -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>) {
    let (tx, rx): (mpsc::Sender<OutputTick>, mpsc::Receiver<OutputTick>) = mpsc::channel();

    let t = thread::spawn(move || {
        let packets = storage::get_all_laps_data();

        let mut context = Context::empty();
        for packet in packets {
            let input_tick = InputTick::from_packet(&packet);
            let result = pipeline::process(&input_tick, &context);
            context = result.new_context;

            tx.send(result.output_tick).ok();
        }
    });

    let r = thread::spawn(move || loop {
        match rx.try_recv() {
            Ok(output_tick) => {
                println!(">> {:?}", output_tick);
            }
            Err(TryRecvError::Disconnected) => {
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    });

    (t, r)
}

//^^ new/refactor

fn receive_tick(rx: &mpsc::Receiver<Tick>) {
    let tick_result = rx.recv().ok();

    if let Some(tick) = tick_result {
        DATA_HOLDER.lock().unwrap().set_data(tick);
    }
}

#[cfg(test)]
pub(crate) mod test_utils {
    use pipeline::types::InputTick;

    pub fn create_input_tick() -> InputTick {
        InputTick {
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
            engine_rate: 90 as f32,
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
            cars_total: 20,
        }
    }
}
