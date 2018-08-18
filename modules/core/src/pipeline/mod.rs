mod routines;
pub(crate) mod types;

use self::types::*;

pub fn process(input_tick: &InputTick, context: &Context) -> PipelineResult {
    let labels = routines::labels::build_labels(input_tick, context);
    let stats = routines::stats::build_stats(input_tick, context, &labels);
    let lap_store_result = routines::lap_storage::store_lap(input_tick, context, &labels, &stats);
    let metadata_store_result = routines::metadata_storage::store_metadata(
        input_tick,
        context,
        &labels,
        &stats,
        &lap_store_result,
    );

    let new_context = routines::context::build_context(&input_tick, &context, &labels, &stats);

    PipelineResult {
        output_tick: OutputTick {
            labels: labels,
            stats: stats,
            lap_store_result: lap_store_result,
            metadata_store_result: metadata_store_result,
        },
        new_context: new_context,
    }
}
