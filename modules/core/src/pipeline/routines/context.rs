use pipeline::input::*;
use pipeline::output::*;

pub fn build_context(tick: &Tick, context: &Context) -> Context {
    Context {
        session_context: SessionContext {
            header: Some(tick.header.clone()),
            current_session: build_session(tick, context),
            current_lap: Some(Lap::from(&tick.lap_data.player)),
            current_sector: Some(Sector::from(&tick.lap_data.player)),
            car_motion: Some(tick.car_motion.player.clone()),
            car_status: build_car_status(tick, context),
        },
        history_context: HistoryContext {},
    }
}

fn build_session(tick: &Tick, context: &Context) -> Option<SessionIdentifier> {
    if tick.session_data.is_some() {
        Some(SessionIdentifier::from(
            tick.session_data.as_ref().unwrap(),
            &tick.header,
        ))
    } else {
        context.session_context.current_session.clone()
    }
}

fn build_car_status(tick: &Tick, context: &Context) -> Option<CarStatus> {
    if tick.car_status.is_some() {
        Some(tick.car_status.as_ref().unwrap().player.clone())
    } else {
        context.session_context.car_status.clone()
    }
}
