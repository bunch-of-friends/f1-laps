mod context;
mod labels;
mod stats;
pub(crate) mod types;

use self::types::*;

struct Pipeline {}
impl DataProcessor for Pipeline {
    fn build_labels(&self, input_tick: &InputTick, context: &Context) -> PacketLabels {
        labels::build_labels(&input_tick, &context)
    }

    fn build_stats(
        &self,
        input_tick: &InputTick,
        context: &Context,
        labels: &PacketLabels,
    ) -> PacketStats {
        stats::build_stats(&input_tick, &context, &labels)
    }

    fn build_context(
        &self,
        input_tick: &InputTick,
        context: &Context,
        labels: &PacketLabels,
    ) -> Context {
        context::build_context(&input_tick, &context, &labels)
    }
}
