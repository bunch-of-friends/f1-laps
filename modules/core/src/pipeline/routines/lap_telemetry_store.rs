use pipeline::input::*;
use pipeline::output::*;

#[derive(Clone)]
pub struct LapTelemetryTempStore {
    pub lap_data: Vec<LapTimeStamped<OptMultiCarData<LapData>>>,
    pub car_status: Vec<LapTimeStamped<OptMultiCarData<CarStatus>>>,
    pub car_telemetry: Vec<LapTimeStamped<OptMultiCarData<CarTelemetry>>>,
    pub car_motion: Vec<LapTimeStamped<OptMultiCarData<CarMotion>>>,
}

#[derive(Clone)]
pub struct LapTimeStamped<T>
where
    T: Clone,
{
    lap_number: u8,
    lap_time: f32,
    data: T,
}

impl LapTelemetryTempStore {
    pub fn new() -> LapTelemetryTempStore {
        LapTelemetryTempStore {
            lap_data: Vec::new(),
            car_status: Vec::new(),
            car_telemetry: Vec::new(),
            car_motion: Vec::new(),
        }
    }
}

pub fn update_temp_store(
    tick: &Tick,
    labels: &Labels,
    events: &Events,
    current_telemetry: &mut LapTelemetryTempStore,
) -> Option<LapTelemetryTempStore> {
    if labels.is_flashback {
        remove_flashback_ticks(current_telemetry, tick);
        return None;
    }

    // if !labels.is_new_lap {
    //     current_telemetry.push(telemetry_item.clone());
    //     return None;
    // }

    // if events.finished_lap.is_some() {
    //     let finished_lap_telemetry = current_telemetry.clone(); //<--- this is slow
    //     current_telemetry.clear();
    //     current_telemetry.push(telemetry_item.clone());
    //     return Some(finished_lap_telemetry);
    // } else {
    //     current_telemetry.clear();
    //     current_telemetry.push(telemetry_item.clone());
    //     return None;
    // }

    None
}

fn remove_flashback_ticks(current_telemetry: &mut LapTelemetryTempStore, tick: &Tick) {
    let flashback_time = tick.lap_data.player.current_lap_time;
    let flashback_lap = tick.lap_data.player.current_lap_number;

    current_telemetry
        .lap_data
        .retain(get_retain_fn(flashback_lap, flashback_time));
    current_telemetry
        .car_status
        .retain(get_retain_fn(flashback_lap, flashback_time));
    current_telemetry
        .car_telemetry
        .retain(get_retain_fn(flashback_lap, flashback_time));
    current_telemetry
        .car_motion
        .retain(get_retain_fn(flashback_lap, flashback_time));
}

fn get_retain_fn<T>(flashback_lap: u8, flashback_time: f32) -> impl Fn(&LapTimeStamped<T>) -> bool
where
    T: Clone,
{
    move |x| -> bool { x.lap_number == flashback_lap && x.lap_time < flashback_time }
}
