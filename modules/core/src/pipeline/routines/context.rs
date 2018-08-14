use pipeline::types::*;

pub(crate) fn build_context(input_tick: &InputTick, context: &Context, labels: &PacketLabels, stats: &PacketStats) -> Context {
    Context {
        session_context: SessionContext {},
        history_context: HistoryContext {},
    }
}
