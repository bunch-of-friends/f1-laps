use pipeline::types::*;

pub fn build_stats(tick: &Tick, context: &Context, labels: &Labels) -> Stats {
    let finished_lap = get_finished_lap(tick, context, labels);
    let finished_sector = get_finished_sector(tick, labels, &finished_lap);
    Stats {
        finished_lap: finished_lap,
        finished_sector: finished_sector,
    }
}

fn get_finished_lap(tick: &Tick, context: &Context, labels: &Labels) -> Option<Lap> {
    if labels.is_new_lap && !labels.is_teleported {
        build_finished_lap(tick, context)
    } else {
        None
    }
}

fn get_finished_sector(tick: &Tick, labels: &Labels, finished_lap: &Option<Lap>) -> Option<Sector> {
    if labels.is_new_sector && !labels.is_teleported {
        build_finished_sector(tick, finished_lap)
    } else {
        None
    }
}

fn build_finished_lap(tick: &Tick, context: &Context) -> Option<Lap> {
    assert!(tick.last_lap_time > 0 as f32);

    let sector_1 = context.session_context.lap.sector_times[0];
    let sector_2 = context.session_context.lap.sector_times[1];

    if sector_1 == 0 as f32 || sector_2 == 0 as f32 {
        return None;
    }

    let finished_lap_time = tick.last_lap_time;
    let sector_3 = finished_lap_time - sector_1 - sector_2;

    Some(Lap::finished(
        sector_1,
        sector_2,
        sector_3,
        finished_lap_time,
        context.session_context.lap.lap_number,
    ))
}

fn build_finished_sector(tick: &Tick, finished_lap: &Option<Lap>) -> Option<Sector> {
    match tick.sector_number {
        1 => {
            if let Some(lap) = finished_lap {
                Some(Sector {
                    sector_number: 3,
                    sector_time: lap.sector_times[2],
                })
            } else {
                None
            }
        }
        2 => {
            if tick.sector1_time > 0 as f32 {
                Some(Sector {
                    sector_number: 1,
                    sector_time: tick.sector1_time,
                })
            } else {
                None
            }
        }
        3 => {
            if tick.sector2_time > 0 as f32 {
                Some(Sector {
                    sector_number: 2,
                    sector_time: tick.sector2_time,
                })
            } else {
                None
            }
        }
        _ => panic!("unexpected sector_number: {}", tick.sector_number),
    }
}
