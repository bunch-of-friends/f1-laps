mod conversion;
mod routines;
pub mod types;

use self::types::*;

pub struct Pipeline {
    context: Context,
    lap_ticks: Vec<Tick>,
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            context: Context::empty(),
            lap_ticks: Vec::new(),
        }
    }

    pub fn process(&mut self, tick: &Tick) -> Output {
        let result = self.process_internal(tick, &self.context);
        self.context = result.0;

        self.update_lap_ticks(tick, &result.1.labels, &result.1.stats);

        result.1
    }

    fn process_internal(&self, tick: &Tick, context: &Context) -> (Context, Output) {
        let labels = routines::labels::build_labels(tick, context);
        let stats = routines::stats::build_stats(tick, context, &labels);
        let new_context = routines::context::build_context(&tick, &context, &labels, &stats);

        let output = Output {
            labels: labels,
            stats: stats,
            tick: tick.clone(),
        };

        (new_context, output)
    }

    fn update_lap_ticks(&mut self, tick: &Tick, labels: &Labels, stats: &Stats) {
        if labels.is_flashback {
            let len = self.lap_ticks.len();

            self.lap_ticks
                .retain(|x| x.lap_number == tick.lap_number && x.lap_time < tick.lap_time);

            let new_len = self.lap_ticks.len();
            assert!(new_len < len);
            println!("removed {}", len - new_len);
        }

        if labels.is_new_lap {
            if let Some(ref _lap) = stats.finished_lap {
                // TODO: store...
                println!("new lap - storing lap ticks, len: {}", self.lap_ticks.len());
            } else {
                println!("new lap - nothing to store");
            }

            self.lap_ticks.clear();
        }

        self.lap_ticks.push(tick.clone());
    }
}
