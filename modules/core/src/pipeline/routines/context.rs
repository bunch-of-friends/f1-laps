use pipeline::types::*;

pub fn build_context(
    input_tick: &InputTick,
    _context: &Context,
    _labels: &Labels,
    _stats: &Stats,
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
