mod conversion;
mod routines;
pub mod types;

use self::types::*;

pub fn process(tick: &Tick, context: &Context) -> (Context, Output) {
    let labels = routines::labels::build_labels(tick, context);
    let stats = routines::stats::build_stats(tick, context, &labels);
    let new_context = routines::context::build_context(&tick, &context, &labels, &stats);

    //TODO: synchronise new_context with persistent storage (e.g: store new best lap)

    let output = Output {
        labels: labels,
        stats: stats,
        tick: tick.clone(),
    };

    (new_context, output)
}
