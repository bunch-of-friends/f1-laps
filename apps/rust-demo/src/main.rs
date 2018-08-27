extern crate f1_laps_core;

use f1_laps_core::prelude::*;

fn main() {
    println!("rust demo is running");

    f1_laps_core::initialise("../../_data-storage".to_string());

    let closure = |output: Output| {
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

        if let Some(ref session_data) = output.session_data {
            println!("{:?}", session_data);
        }

        if let Some(ref car_telemetry) = output.car_telemetry {
            println!("{:?}", car_telemetry);
        }

        if let Some(ref car_status) = output.car_status {
            println!("{:?}", car_status);
        }

        if let Some(ref car_motion) = output.car_motion {
            println!("{:?}", car_motion);
        }
    };

    let h = f1_laps_core::start_listening(20777, closure);

    assert!(!h.0.join().is_err());
    assert!(!h.1.join().is_err());
}
