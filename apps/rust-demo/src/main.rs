#![allow(unknown_lints)]
#![deny(clippy)]

extern crate f1_laps_core;

use f1_laps_core::prelude::*;
use f1_laps_core::Context;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let context = f1_laps_core::initialise("../../_data-storage");
    let context: &'static Context = Box::leak(context);

    // let x = f1_laps_core::get_laps_headers(context);
    // println!("{:?}", x);

    // let y = f1_laps_core::get_lap_telemetry(context, x.first().unwrap().id.clone());
    // println!("{:?}", y);

    if let Some(h) = start(&args, context) {
        assert!(!h.0.join().is_err());
        assert!(!h.1.join().is_err());
    }
}

fn start(
    args: &[String],
    context: &'static Context,
) -> Option<(std::thread::JoinHandle<()>, std::thread::JoinHandle<()>)> {
    let mut mode_value = "";
    let mut b_value = true;

    if args.len() > 1 {
        mode_value = args[1].as_str();
    }

    if args.len() > 2 {
        b_value = args[2] != "false" && args[2] != "f";
    };

    match mode_value {
        "replay" | "r" => {
            println!("running in replay mode");
            Some(f1_laps_core::replay_packets(
                context,
                b_value,
                false,
                on_received,
            ))
        }
        "udp" | "u" => {
            println!("running in udp mode");
            Some(f1_laps_core::start_listening(
                context,
                20777,
                b_value,
                on_received,
            ))
        }
        _ => {
            println!("no mode selected");
            println!("---> run with argument 'replay' or 'r'  to replay stored packets, optionaly use argument 'false' to disable time simulation");
            println!("---> run with argument 'udp' or 'u' to listen for incoming udp packets, optionaly use argument 'false' to disable packet storing");
            None
        }
    }
}

#[allow(needless_pass_by_value)]
fn on_received(output: Output) {
    if output.labels.is_flashback {
        println!("flashback");
    }

    if output.labels.is_teleported {
        println!("teleported");
    }

    // if let Some(ref participants_info) = output.participants_info {
    //     println!("PARTICIPANTS >>> {:?}", participants_info);
    // }

    if let Some(ref session) = output.events.started_session {
        println!("SESSION STARTER >>> {:?}", session);
    }

    if let Some(ref sector) = output.events.finished_sector {
        println!("SECTOR FINISHED >>> {:?}", sector);
    }

    if let Some(ref lap) = output.events.finished_lap {
        println!("LAP FINISHED >>> {:?}", lap);
    }

    // println!(
    //     "L{} {}/3 {}",
    //     output.lap_data.player.current_lap_number,
    //     output.lap_data.player.current_sector_number,
    //     output.lap_data.player.current_lap_time
    // );

    // if let Some(ref car_telemetry) = output.car_telemetry {
    //     println!("{:?}", car_telemetry);
    // }

    // if let Some(ref car_status) = output.car_status {
    //     println!("{:?}", car_status);
    // }

    // if let Some(ref car_motion) = output.car_motion {
    //     println!("{:?}", car_motion);
    // }
}
