use pipeline::types::*;

pub fn build_events(tick: &Tick, context: &Context, labels: &Labels) -> Events {
    let started_session = get_started_session(tick, labels);
    let finished_lap = get_finished_lap(tick, context, labels);
    let finished_sector = get_finished_sector(tick, labels, &finished_lap);

    Events {
        started_session: started_session,
        finished_lap: finished_lap,
        finished_sector: finished_sector,
    }
}

fn get_started_session(tick: &Tick, labels: &Labels) -> Option<Session> {
    if labels.is_new_session & tick.session_info.is_some() {
        Session::from_tick(tick)
    } else {
        None
    }
}

fn get_finished_lap(tick: &Tick, context: &Context, labels: &Labels) -> Option<Lap> {
    if labels.is_new_lap && tick.lap_data.is_some() && !labels.is_teleported {
        build_finished_lap(tick.lap_data.as_ref().unwrap(), context)
    } else {
        None
    }
}

fn get_finished_sector(tick: &Tick, labels: &Labels, finished_lap: &Option<Lap>) -> Option<Sector> {
    if labels.is_new_sector && tick.lap_data.is_some() && !labels.is_teleported {
        build_finished_sector(tick.lap_data.as_ref().unwrap(), finished_lap)
    } else {
        None
    }
}

fn build_finished_lap(lap_data: &LapData, context: &Context) -> Option<Lap> {
    assert!(lap_data.last_lap_time > 0 as f32);

    let sector_1 = context.session_context.lap.sector_times[0];
    let sector_2 = context.session_context.lap.sector_times[1];

    if sector_1 == 0 as f32 || sector_2 == 0 as f32 {
        return None;
    }

    let finished_lap_time = lap_data.last_lap_time;
    let sector_3 = finished_lap_time - sector_1 - sector_2;

    Some(Lap::finished(
        sector_1,
        sector_2,
        sector_3,
        finished_lap_time,
        context.session_context.lap.lap_number,
    ))
}

fn build_finished_sector(lap_data: &LapData, finished_lap: &Option<Lap>) -> Option<Sector> {
    match lap_data.current_sector_number {
        1 => {
            if let Some(lap) = finished_lap {
                Some(Sector::finished(lap.sector_times[2], 3))
            } else {
                None
            }
        }
        2 => {
            if lap_data.sector1_time > 0 as f32 {
                Some(Sector::finished(lap_data.sector1_time, 1))
            } else {
                None
            }
        }
        3 => {
            if lap_data.sector2_time > 0 as f32 {
                Some(Sector::finished(lap_data.sector2_time, 2))
            } else {
                None
            }
        }
        _ => panic!(
            "unexpected sector_number: {}",
            lap_data.current_sector_number
        ),
    }
}
