mod routines;
pub mod types;

use self::types::*;

pub fn process(input_tick: &InputTick, context: &Context) -> PipelineResult {
    let labels = routines::labels::build_labels(input_tick, context);
    let stats = routines::stats::build_stats(input_tick, context, &labels);
    let new_context = routines::context::build_context(&input_tick, &context, &labels, &stats);

    //TODO: synchronise new_context with persistent storage (e.g: store new best lap)

    PipelineResult {
        output_tick: OutputTick {
            labels: labels,
            stats: stats
        },
        new_context: new_context,
    }
}
