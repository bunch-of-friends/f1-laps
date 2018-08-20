use pipeline::types::*;

pub fn build_context(
    tick: &Tick,
    _context: &Context,
    _labels: &Labels,
    _stats: &Stats,
) -> Context {
    Context {
        session_context: SessionContext {
            session: Session::from_tick(tick),
            lap: Lap::from_tick(tick),
            sector: Sector::from_tick(tick),
            position: Position::from_tick(tick),
        },
        history_context: HistoryContext {},
    }
}
