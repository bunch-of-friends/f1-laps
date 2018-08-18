extern crate f1_laps_core;

mod common;

use f1_laps_core::pipeline::types::OutputTick;

#[test]
fn replay_all_laps_test() {
    common::setup();

    let closure = |tick: &OutputTick| {
        // println!("tick received >> {:?}", tick);

        // if tick.labels.current_session.session_time < time {
        //     println!("flashback :/");
        //     time = tick.labels.current_session.session_time;
        // }

        // println!("{}", tick.labels.current_session.session_time);

        if tick.labels.is_flashback {
            println!("flashback");
        }

        if tick.labels.is_teleported {
            println!("teleported");
        }

        if let Some(ref sector) = tick.stats.finished_sector {
            println!("{:?}", sector);
        }

        if let Some(ref lap) = tick.stats.finished_lap {
            println!("{:?}", lap);
        }
    };

    let h = f1_laps_core::replay_all_laps_new(closure);

    assert!(!h.0.join().is_err());
    assert!(!h.1.join().is_err());
}
