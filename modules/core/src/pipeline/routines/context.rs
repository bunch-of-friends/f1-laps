use pipeline::types::*;

pub fn build_context(
    input_tick: &InputTick,
    context: &Context,
    labels: &PacketLabels,
    stats: &PacketStats,
) -> Context {
    Context {
        session_context: SessionContext {
            session: Some(Session::from_input_tick(input_tick)),
            lap_number: input_tick.lap_number,
            sector: input_tick.sector,
        },
        history_context: HistoryContext {},
    }
}
