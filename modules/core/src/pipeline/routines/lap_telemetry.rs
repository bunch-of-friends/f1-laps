use pipeline::input::*;
use pipeline::output::*;

pub struct LapTelemetryTempStore {
    pub lap_data: Vec<LapTimeStamped<MultiCarData<LapData>>>,
    pub car_status: Vec<LapTimeStamped<MultiCarData<CarStatus>>>,
    pub car_telemetry: Vec<LapTimeStamped<MultiCarData<CarTelemetry>>>,
    pub car_motion: Vec<LapTimeStamped<MultiCarData<CarMotion>>>,
}

pub struct LapTimeStamped<T> {
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

    pub fn add_tick(&mut self, tick: &Tick) {
        let (lap_number, lap_time) = get_tick_timestamp(tick);

        self.lap_data.push(get_timestamped(tick.lap_data.clone(), lap_number, lap_time));
        self.car_telemetry.push(get_timestamped(tick.car_telemetry.clone(), lap_number, lap_time));
        self.car_motion.push(get_timestamped(tick.car_motion.clone(), lap_number, lap_time));

        if let Some(ref car_status) = tick.car_status {
            self.car_status.push(get_timestamped(car_status.clone(), lap_number, lap_time));
        }
    }

    pub fn remove_flashback_ticks(&mut self, tick: &Tick) {
        let (lap_number, lap_time) = get_tick_timestamp(tick);

        self.lap_data.retain(get_retain_fn(lap_number, lap_time));
        self.car_status.retain(get_retain_fn(lap_number, lap_time));
        self.car_telemetry.retain(get_retain_fn(lap_number, lap_time));
        self.car_motion.retain(get_retain_fn(lap_number, lap_time));
    }

    pub fn cut_off(&mut self) -> LapTelemetryTempStore {
        LapTelemetryTempStore {
            lap_data: self.lap_data.split_off(0),
            car_status: self.car_status.split_off(0),
            car_telemetry: self.car_telemetry.split_off(0),
            car_motion: self.car_motion.split_off(0),
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
        current_telemetry.remove_flashback_ticks(tick);
        return None;
    }

    if !labels.is_new_lap {
        current_telemetry.add_tick(tick);
        return None;
    }

    let finished_lap_telemetry = current_telemetry.cut_off();
    current_telemetry.add_tick(tick);

    if events.finished_lap.is_some() {
        return Some(finished_lap_telemetry);
    } else {
        return None;
    }
}

fn get_tick_timestamp(tick: &Tick) -> (u8, f32) {
    (tick.lap_data.player.current_lap_number, tick.lap_data.player.current_lap_time)
}

fn get_timestamped<T>(data: T, lap_number: u8, lap_time: f32) -> LapTimeStamped<T> {
    LapTimeStamped {
        lap_number: lap_number,
        lap_time: lap_time,
        data: data,
    }
}

fn get_retain_fn<T>(flashback_lap: u8, lap_time: f32) -> impl Fn(&LapTimeStamped<T>) -> bool {
    move |x| -> bool { x.lap_number == flashback_lap && x.lap_time < lap_time }
}
