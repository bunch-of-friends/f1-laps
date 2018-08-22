use pipeline::types::*;

pub fn update_lap_ticks(
    tick: &Tick,
    labels: &Labels,
    events: &Events,
    current_ticks: &mut Vec<Tick>,
) -> Option<Vec<Tick>> {
    if labels.is_flashback {
        remove_jumped_ticks(current_ticks, tick);
        return None;
    }

    if !labels.is_new_lap {
        current_ticks.push(tick.clone());
        return None;
    }

    if events.finished_lap.is_some() {
        let finished_lap_ticks = current_ticks.clone(); //<--- this is slow
        current_ticks.clear();
        current_ticks.push(tick.clone());
        return Some(finished_lap_ticks);
    } else {
        current_ticks.clear();
        current_ticks.push(tick.clone());
        return None;
    }
}

pub fn remove_jumped_ticks(ticks: &mut Vec<Tick>, tick: &Tick) {
    ticks.retain(|x| x.lap_number == tick.lap_number && x.lap_time < tick.lap_time);
}
