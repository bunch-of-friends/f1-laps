use pipeline::input::*;
use pipeline::output::*;

pub fn build_context(tick: &Tick, context: &Context) -> Context {
    Context {
        session_context: SessionContext {
            header: Some(tick.header.clone()),
            session: build_session(tick, context),
            lap: build_lap(tick, context),
            sector: build_sector(tick, context),
            car_motion: build_car_motion(tick, context),
            car_status: build_car_status(tick, context),
        },
        history_context: HistoryContext {},
    }
}

fn build_session(tick: &Tick, context: &Context) -> Option<SessionIdentifier> {
    if tick.session_data.is_some() {
        Some(SessionIdentifier::from_session_data(
            tick.session_data.as_ref().unwrap(),
            &tick.header,
        ))
    } else {
        context.session_context.session.clone()
    }
}

fn build_lap(tick: &Tick, context: &Context) -> Option<Lap> {
    if let Some(lap) = Lap::from_tick(tick) {
        Some(lap)
    } else {
        context.session_context.lap.clone()
    }
}

fn build_sector(tick: &Tick, context: &Context) -> Option<Sector> {
    if let Some(sector) = Sector::from_tick(tick) {
        Some(sector)
    } else {
        context.session_context.sector.clone()
    }
}

fn build_car_motion(tick: &Tick, context: &Context) -> Option<CarMotion> {
    if tick.car_motion.is_some() {
        tick.car_motion.clone()
    } else {
        context.session_context.car_motion.clone()
    }
}

fn build_car_status(tick: &Tick, context: &Context) -> Option<CarStatus> {
    if tick.car_status.is_some() {
        tick.car_status.clone()
    } else {
        context.session_context.car_status.clone()
    }
}
