use pipeline::types::*;

pub fn build_context(
    input_tick: &InputTick,
    context: &Context,
    labels: &Labels,
    stats: &Stats,
) -> Context {
    Context {
        session_context: SessionContext {
            session: Session::from_input_tick(input_tick),
            lap: Lap::from_input_tick(input_tick),
            sector: Sector::from_input_tick(input_tick),
        },
        history_context: HistoryContext {},
    }
}
