mod conversion;
mod routines;
pub mod types;

use std::thread;

use self::types::*;
use lap_metadata::LapMetadata;
use storage;

pub struct Pipeline {
    context: Context,
    current_lap_ticks: Vec<Tick>,
    should_store_laps: bool,
    should_wait_for_fs: bool,
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            context: Context::empty(),
            current_lap_ticks: Vec::new(),
            should_store_laps: true,
            should_wait_for_fs: false,
        }
    }

    pub fn set_should_store_laps(&mut self, should_store_laps: bool) {
        self.should_store_laps = should_store_laps;
    }

    pub fn set_should_wait_for_fs(&mut self, should_wait_for_fs: bool) {
        self.should_wait_for_fs = should_wait_for_fs;
    }

    pub fn process(&mut self, tick: &Tick) -> Output {
        let labels = routines::labels::build_labels(tick, &self.context);
        let events = routines::events::build_events(tick, &self.context, &labels);

        let finished_lap_ticks = routines::lap_ticks::update_lap_ticks(
            tick,
            &labels,
            &events,
            &mut self.current_lap_ticks,
        );

        if self.should_store_laps {
            self.try_store_lap(finished_lap_ticks, &labels, &events);
        }

        let new_context = routines::context::build_context(&tick, &self.context, &labels);

        self.context = new_context;

        Output {
            labels: labels,
            events: events,
            tick: tick.clone(),
        }
    }

    fn try_store_lap(
        &self,
        finished_lap_ticks: Option<Vec<Tick>>,
        labels: &Labels,
        events: &Events,
    ) {
        if let Some(ticks) = finished_lap_ticks {
            if let Some(ref finished_lap) = events.finished_lap {
                let metadata = LapMetadata::new(
                    &self.context.session_context.session,
                    finished_lap,
                    labels.tyre_compound,
                );

                let t = thread::spawn(move || {
                    storage::store_lap(ticks, &metadata);
                });

                if self.should_wait_for_fs {
                    let j = t.join();
                    assert!(!j.is_err());
                }
            }
        }
    }
}
