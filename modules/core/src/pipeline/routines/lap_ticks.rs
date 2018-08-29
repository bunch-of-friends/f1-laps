use pipeline::input::*;
use pipeline::output::*;

pub fn update_lap_ticks(
    tick: &Tick,
    labels: &Labels,
    events: &Events,
    current_telemetry: &mut Vec<CarTelemetry>,
) -> Option<Vec<CarTelemetry>> {
    if tick.lap_data.is_none() {
        return None;
    }

    let telemetry_item = tick.car_telemetry.as_ref().unwrap();

    if labels.is_flashback {
        remove_jumped_ticks(current_telemetry, telemetry_item);
        return None;
    }

    if !labels.is_new_lap {
        current_telemetry.push(telemetry_item.clone());
        return None;
    }

    if events.finished_lap.is_some() {
        let finished_lap_telemetry = current_telemetry.clone(); //<--- this is slow
        current_telemetry.clear();
        current_telemetry.push(telemetry_item.clone());
        return Some(finished_lap_telemetry);
    } else {
        current_telemetry.clear();
        current_telemetry.push(telemetry_item.clone());
        return None;
    }
}

pub fn remove_jumped_ticks(
    current_telemetry: &mut Vec<CarTelemetry>,
    telemetry_item: &CarTelemetry,
) {
    // current_telemetry.retain(|x| {
    //     x.current_lap_number == lap_data.current_lap_number
    //         && x.current_lap_time < lap_data.current_lap_time
    // });
}
