extern crate f1_laps_core;

mod common;

use std::thread;

#[test]
fn replay_all_laps_test() {
    common::setup();

    f1_laps_core::replay_all_laps_new();

    thread::spawn(move || {
        subscribe_to_ticks();
    });

    thread::park_timeout(std::time::Duration::from_secs(30));
}

fn subscribe_to_ticks() {
    loop {
        if let Some(tick) = f1_laps_core::get_next_tick() {
            if let Some(session) = tick.session_started {
                println!("session started >> {:?}", session);
            }

            if let Some(sector) = tick.sector_finished {
                println!("sector finished >> {:?}", sector);
            }

            if let Some(lap) = tick.lap_finished {
                println!("lap finished >> {:?}", lap);
            }

            println!("tick >> {:?}", tick.live_data.current_speed);
        }
        std::thread::park_timeout(std::time::Duration::from_millis(30));
    }
}
