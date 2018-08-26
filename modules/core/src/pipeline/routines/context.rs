use pipeline::types::*;

pub fn build_context(tick: &Tick, context: &Context) -> Context {
    Context {
        session_context: SessionContext {
            session: Session::from_tick(tick).unwrap_or(context.session_context.session.clone()),
            lap: Lap::from_tick(tick).unwrap_or(context.session_context.lap.clone()),
            sector: Sector::from_tick(tick).unwrap_or(context.session_context.sector.clone()),
            car_motion: tick.car_motion.unwrap_or(context.session_context.car_motion.clone()),
            car_status: CarStatus::from_tick(tick)
                .unwrap_or(context.session_context.car_status.clone()),
        },
        history_context: HistoryContext {},
    }
}
