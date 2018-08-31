extern crate f1_laps_core;

use f1_laps_core::prelude::*;

fn main() {
    println!("rust demo is running");

    f1_laps_core::initialise("../../_data-storage".to_string());

    let on_received = |output: Output| {
        if output.labels.is_flashback {
            println!("flashback");
        }

        if output.labels.is_teleported {
            println!("teleported");
        }

        if let Some(ref sector) = output.events.finished_sector {
            println!("{:?}", sector);
        }

        if let Some(ref lap) = output.events.finished_lap {
            println!("{:?}", lap);
        }

        if let Some(ref lap_data) = output.lap_data {
            println!("L{} {}/3", lap_data.current_lap_number, lap_data.current_sector_number);
        }

        // if let Some(ref car_telemetry) = output.car_telemetry {
        //     println!("{:?}", car_telemetry);
        // }

        // if let Some(ref car_status) = output.car_status {
        //     println!("{:?}", car_status);
        // }

        // if let Some(ref car_motion) = output.car_motion {
        //     println!("{:?}", car_motion);
        // }
    };

    let h = f1_laps_core::start_listening(20777, true, on_received);
    // let h = f1_laps_core::replay_packets(on_received);

    assert!(!h.0.join().is_err());
    assert!(!h.1.join().is_err());
}
