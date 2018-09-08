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
            car_status: try_update_from_tick(tick.car_status.as_ref(), context.session_context.car_status.as_ref()),
            car_setup: try_update_from_tick(tick.car_setup.as_ref(), context.session_context.car_setup.as_ref()),
            participants_info: try_update_from_tick(tick.participants_info.as_ref(), context.session_context.participants_info.as_ref()),
            session_data: build_session_data(tick, context),
        },
        history_context: HistoryContext {},
    }
}

fn build_session(tick: &Tick, context: &Context) -> Option<SessionIdentifier> {
    if tick.session_data.is_some() {
        Some(SessionIdentifier::from(tick.session_data.as_ref().unwrap(), &tick.header))
    } else {
        context.session_context.current_session.clone()
    }
}

fn build_session_data(tick: &Tick, context: &Context) -> Option<SessionData> {
    if tick.session_data.is_some() {
        tick.session_data.clone()
    } else {
        context.session_context.session_data.clone()
    }
}

fn try_update_from_tick<T>(tick: Option<&MultiCarData<T>>, context: Option<&T>) -> Option<T>
where
    T: Clone,
{
    if tick.is_some() {
        Some(tick.unwrap().player.clone())
    } else {
        Some(context.unwrap().clone())
    }
}
