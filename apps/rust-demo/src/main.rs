extern crate f1_laps_core;

use f1_laps_core::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    f1_laps_core::initialise("../../_data-storage".to_string());

    let h = start(&args);

    assert!(!h.0.join().is_err());
    assert!(!h.1.join().is_err());
}

fn start(args: &Vec<String>) -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>) {
    let mut mode_value = "";
    let mut b_value = true;

    if args.len() > 1 {
        mode_value = args[1].as_str();
    };

    if args.len() > 2 {
        b_value = args[2] != "false" && args[2] != "f";
    };

    if mode_value == "replay" || mode_value == "r" {
        f1_laps_core::replay_packets(b_value, on_received)
    } else {
        f1_laps_core::start_listening(20777, b_value, on_received)
    }

    
}

fn on_received(output: Output) {
    // if output.labels.is_flashback {
    //     println!("flashback");
    // }

    // if output.labels.is_teleported {
    //     println!("teleported");
    // }

    // if let Some(ref sector) = output.events.finished_sector {
    //     println!("{:?}", sector);
    // }

    // if let Some(ref lap) = output.events.finished_lap {
    //     println!("{:?}", lap);
    // }

    // if let Some(ref lap_data) = output.lap_data {
    //     println!(
    //         "L{} {}/3",
    //         lap_data.current_lap_number, lap_data.current_sector_number
    //     );
    // }

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
