// extern crate f1_laps_core;

// mod common;

// use f1_laps_core::prelude::*;

// #[test]
// fn replay_packets_test() {
//     common::setup();

//     let closure = |output: Output| {
//         // println!("output received >> {:?}", output);

//         // if output.labels.current_session.session_time < time {
//         //     println!("flashback :/");
//         //     time = output.labels.current_session.session_time;
//         // }

//         // println!("{}", output.labels.current_session.session_time);

//         if output.labels.is_flashback {
//             println!("flashback");
//         }

//         if output.labels.is_teleported {
//             println!("teleported");
//         }

//         if let Some(ref sector) = output.events.finished_sector {
//             println!("{:?}", sector);
//         }

//         if let Some(ref lap) = output.events.finished_lap {
//             println!("{:?}", lap);
//         }
//     };

//     let h = f1_laps_core::replay_packets(false, closure);

//     assert!(!h.0.join().is_err());
//     assert!(!h.1.join().is_err());
// }
