use chrono::Utc;
use pipeline::input::*;
use pipeline::output::*;
use storage::models::{LapHeader, LapTelemetry};
use storage::Storage;
use uuid::Uuid;

pub(crate) struct LapTelemetryTempStore {
    pub lap_data: Vec<LapTimeStamped<MultiCarData<LapData>>>,
    pub car_status: Vec<LapTimeStamped<MultiCarData<CarStatus>>>,
    pub car_telemetry: Vec<LapTimeStamped<MultiCarData<CarTelemetry>>>,
    pub car_motion: Vec<LapTimeStamped<MultiCarData<CarMotion>>>,
}

pub(crate) struct LapTimeStamped<T> {
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

pub(crate) fn update_temp_store(
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
        Some(finished_lap_telemetry)
    } else {
        None
    }
}

pub(crate) fn try_store_lap(storage: &'static Storage, finished_lap_telemetry: Option<LapTelemetryTempStore>, events: &Events, context: &Context) {
    if finished_lap_telemetry.is_none() {
        return;
    }

    if events.finished_lap.is_none()
        || context.session_context.current_session.is_none()
        || context.session_context.session_data.is_none()
        || context.session_context.participants_info.is_none()
        || context.session_context.car_status.is_none()
        || context.session_context.car_setup.is_none()
    {
        println!("would store a lap but data incomplete");
        return;
    }

    let (lap_header, lap_telemetry) = get_storage_items(
        finished_lap_telemetry.unwrap(),
        context.session_context.current_session.as_ref().unwrap(),
        context.session_context.session_data.as_ref().unwrap(),
        context.session_context.participants_info.as_ref().unwrap(),
        context.session_context.car_status.as_ref().unwrap(),
        context.session_context.car_setup.as_ref().unwrap(),
        events.finished_lap.as_ref().unwrap(),
    );

    storage.lap_headers.set(&lap_header.id, &lap_header);
    storage.lap_telemetry.set(&lap_telemetry.id, &lap_telemetry);
}

fn get_storage_items(
    finished_lap_telemetry: LapTelemetryTempStore,
    session: &SessionIdentifier,
    session_data: &SessionData,
    participants_info: &ParticipantInfo,
    car_status: &CarStatus,
    car_setup: &CarSetup,
    lap: &Lap,
) -> (LapHeader, LapTelemetry) {
    assert!(lap.lap_time > 0f32);
    assert!(lap.sector_times[0] > 0f32);
    assert!(lap.sector_times[1] > 0f32);
    assert!(lap.sector_times[2] > 0f32);

    let lap_header = LapHeader {
        id: Uuid::new_v4().to_string(),
        recorded_date: Utc::now(),
        track_id: session.track_id,
        team_id: participants_info.team_id,
        era: session.era,
        tyre_compound: car_status.tyre_compound,
        weather: session_data.weather,
        session_type: session.session_type,
        lap_number: lap.lap_number,
        lap_time: lap.lap_time,
        sector_times: lap.sector_times,
        note: String::new(),
    };

    let lap_telemetry = LapTelemetry {
        id: lap_header.id.clone(),
        session_data: session_data.clone(),
        lap_data: to_player_vector(finished_lap_telemetry.lap_data),
        car_status: to_player_vector(finished_lap_telemetry.car_status),
        car_telemetry: to_player_vector(finished_lap_telemetry.car_telemetry),
        car_motion: to_player_vector(finished_lap_telemetry.car_motion),
        car_setup: car_setup.clone(),
        participants_info: participants_info.clone(),
    };

    (lap_header, lap_telemetry)
}

fn to_player_vector<T>(input: Vec<LapTimeStamped<MultiCarData<T>>>) -> Vec<T>
where
    T: Clone,
{
    input.into_iter().map(|x| x.data.player).collect()
}

fn get_tick_timestamp(tick: &Tick) -> (u8, f32) {
    (tick.lap_data.player.current_lap_number, tick.lap_data.player.current_lap_time)
}

fn get_timestamped<T>(data: T, lap_number: u8, lap_time: f32) -> LapTimeStamped<T> {
    LapTimeStamped { lap_number, lap_time, data }
}

fn get_retain_fn<T>(flashback_lap: u8, lap_time: f32) -> impl Fn(&LapTimeStamped<T>) -> bool {
    move |x| -> bool { x.lap_number == flashback_lap && x.lap_time < lap_time }
}
