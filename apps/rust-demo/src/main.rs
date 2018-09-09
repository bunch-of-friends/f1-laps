extern crate f1_laps_core;

use f1_laps_core::Context;
use f1_laps_core::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let context = f1_laps_core::initialise("../../_data-storage".to_string());
    let static_ref: &'static Context = Box::leak(context);

    if let Some(h) = start(&args, static_ref) {
        assert!(!h.0.join().is_err());
        assert!(!h.1.join().is_err());
    }
}

fn start(
    args: &Vec<String>,
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
            Some(f1_laps_core::replay_packets(context, b_value, false, on_received))
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

fn on_received(output: Output) {
    if output.labels.is_flashback {
        println!("flashback");
    }

    if output.labels.is_teleported {
        println!("teleported");
    }

    if let Some(ref participants_info) = output.participants_info {
        println!("PARTICIPANTS >>> {:?}", participants_info);
    }

    if let Some(ref sector) = output.events.finished_sector {
        println!("SECTOR FINISHED >>> {:?}", sector);
    }

    if let Some(ref lap) = output.events.finished_lap {
        println!("LAP FINISHED >>> {:?}", lap);
    }

    println!(
        "L{} {}/3 {}",
        output.lap_data.player.current_lap_number,
        output.lap_data.player.current_sector_number,
        output.lap_data.player.current_lap_time
    );

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
