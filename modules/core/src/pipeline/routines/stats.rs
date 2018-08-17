use pipeline::types::*;

pub fn build_stats(input_tick: &InputTick, context: &Context, labels: &Labels) -> Stats {
    let finished_lap = get_finished_lap(input_tick, context, labels);
    let finished_sector = get_finished_sector(input_tick, context, labels, &finished_lap);
    Stats {
        finished_lap: finished_lap,
        finished_sector: finished_sector,
    }
}

fn get_finished_lap(input_tick: &InputTick, context: &Context, labels: &Labels) -> Option<Lap> {
    if labels.is_new_lap {
        Some(build_finished_lap(input_tick, context))
    } else {
        None
    }
}

fn get_finished_sector(
    input_tick: &InputTick,
    context: &Context,
    labels: &Labels,
    finished_lap: &Option<Lap>,
) -> Option<Sector> {
    if labels.is_new_sector {
        Some(build_finished_sector(input_tick, context, finished_lap))
    } else {
        None
    }
}

fn build_finished_lap(input_tick: &InputTick, context: &Context) -> Lap {
    assert!(input_tick.last_lap_time > 0 as f32);
    assert!(context.session_context.lap.sector_times[0] > 0 as f32);
    assert!(context.session_context.lap.sector_times[1] > 0 as f32);
    assert!(context.session_context.lap.sector_times[2] == 0 as f32);

    let finished_lap_time = input_tick.last_lap_time;
    let finished_lap_s3_t = finished_lap_time
        - context.session_context.lap.sector_times[0]
        - context.session_context.lap.sector_times[1];

    Lap {
        lap_number: context.session_context.lap.lap_number,
        lap_time: finished_lap_time,
        sector_times: [
            context.session_context.lap.sector_times[0],
            context.session_context.lap.sector_times[1],
            finished_lap_s3_t,
        ],
    }
}

fn build_finished_sector(
    input_tick: &InputTick,
    _: &Context,
    finished_lap: &Option<Lap>,
) -> Sector {
    // sector 3 finished
    if let Some(lap) = finished_lap {
        Sector {
            sector_number: 3,
            sector_time: lap.sector_times[2],
        }
    // either sector 1 or 2 finished
    } else {
        assert!(input_tick.sector_number == 1 || input_tick.sector_number == 2);

        if input_tick.sector_number == 1 {
            Sector {
                sector_number: 1,
                sector_time: input_tick.sector1_time,
            }
        } else {
            Sector {
                sector_number: 2,
                sector_time: input_tick.sector2_time,
            }
        }
    }
}
