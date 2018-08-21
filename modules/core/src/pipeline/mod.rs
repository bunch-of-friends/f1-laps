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
}
