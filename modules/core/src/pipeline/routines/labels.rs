use pipeline::types::*;

pub fn build_labels(input_tick: &InputTick, context: &Context) -> Labels {
    let current_session = Session::from_input_tick(&input_tick);
    let current_lap = Lap::from_input_tick(&input_tick);

    let is_new_session = is_new_session(&current_session, context);
    let is_new_lap = is_new_lap(is_new_session, input_tick, context);
    let is_new_sector = is_new_sector(is_new_lap, input_tick, context);
    let is_flashback = is_flashback(is_new_session, is_new_lap, &current_lap, context);
    let is_teleported = is_teleported(is_new_session, input_tick, context);

    Labels {
        is_new_session: is_new_session,
        is_new_lap: is_new_lap,
        is_new_sector: is_new_sector,
        is_flashback: is_flashback,
        is_teleported: is_teleported,
        current_session: current_session,
        current_lap: current_lap,
        current_sector: Sector::from_input_tick(&input_tick),
    }
}

fn is_new_session(session: &Session, context: &Context) -> bool {
    !session.eq(&context.session_context.session)
}

fn is_new_lap(is_new_session: bool, input_tick: &InputTick, context: &Context) -> bool {
    if is_new_session {
        true
    } else {
        input_tick.lap_number != context.session_context.lap.lap_number
    }
}

fn is_new_sector(is_new_lap: bool, input_tick: &InputTick, context: &Context) -> bool {
    if is_new_lap {
        true
    } else {
        input_tick.sector_number != context.session_context.sector.sector_number
    }
}

fn is_flashback(
    is_new_session: bool,
    is_new_lap: bool,
    current_lap: &Lap,
    context: &Context,
) -> bool {
    if is_new_session || is_new_lap {
        false
    } else {
        context.session_context.lap.lap_time > current_lap.lap_time
    }
}

fn is_teleported(is_new_session: bool, input_tick: &InputTick, context: &Context) -> bool {
    if is_new_session {
        return false;
    }

    let x_diff = (input_tick.x - context.session_context.position.x).abs();
    let y_diff = (input_tick.y - context.session_context.position.y).abs();
    let z_diff = (input_tick.z - context.session_context.position.z).abs();

    let max = 10 as f32;

    x_diff > max || y_diff > max || z_diff > max
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils;

    fn create_input() -> (InputTick, Context) {
        (
            test_utils::create_input_tick(),
            Context {
                session_context: SessionContext::empty(),
                history_context: HistoryContext {},
            },
        )
    }

    #[test]
    fn is_new_sector_test() {
        let i = create_input();
        let mut tick = i.0;
        let mut context = i.1;

        assert_eq!(true, is_new_sector(true, &tick, &context));

        context.session_context.sector.sector_number = 0;
        tick.sector_number = 0;
        assert_eq!(false, is_new_sector(false, &tick, &context));

        context.session_context.sector.sector_number = 0;
        tick.sector_number = 1;
        assert_eq!(true, is_new_sector(false, &tick, &context));
    }

    #[test]
    fn is_new_lap_test() {
        let i = create_input();
        let mut tick = i.0;
        let mut context = i.1;

        assert_eq!(true, is_new_lap(true, &tick, &context));

        context.session_context.lap.lap_number = 1;
        tick.lap_number = 1;
        assert_eq!(false, is_new_lap(false, &tick, &context));

        context.session_context.lap.lap_number = 1;
        tick.lap_number = 2;
        assert_eq!(true, is_new_lap(false, &tick, &context));
    }

    #[test]
    fn is_new_session_test() {
        let i = create_input();
        let mut session = Session::from_input_tick(&i.0);
        let mut context = i.1;

        context.session_context = SessionContext::empty();
        assert_eq!(true, is_new_session(&session, &context));

        context.session_context.session = session.clone();
        assert_eq!(false, is_new_session(&session, &context));

        let mut s = session.clone();
        s.team_id = 10;
        context.session_context.session = s;
        session.team_id = 9;
        assert_eq!(true, is_new_session(&session, &context));
    }

    #[test]
    fn is_flashback_test() {
        let i = create_input();
        let mut lap = Lap::from_input_tick(&i.0);
        let mut context = i.1;

        context.session_context.lap.lap_time = 100.12;
        lap.lap_time = 150.15;
        assert_eq!(false, is_flashback(false, false, &lap, &context));
        assert_eq!(false, is_flashback(true, true, &lap, &context));
        assert_eq!(false, is_flashback(false, true, &lap, &context));
        assert_eq!(false, is_flashback(true, false, &lap, &context));

        context.session_context.lap.lap_time = 100.12;
        lap.lap_time = 50.15;
        assert_eq!(true, is_flashback(false, false, &lap, &context));
        assert_eq!(false, is_flashback(true, true, &lap, &context));
        assert_eq!(false, is_flashback(false, true, &lap, &context));
        assert_eq!(false, is_flashback(true, false, &lap, &context));
    }

    #[test]
    fn is_teleported_test() {
        let i = create_input();
        let mut tick = i.0;
        let mut context = i.1;

        context.session_context.position.x = 1 as f32;
        context.session_context.position.y = 1 as f32;
        context.session_context.position.z = 1 as f32;
        tick.x = 500 as f32;
        tick.y = 500 as f32;
        tick.z = 500 as f32;
        assert_eq!(false, is_teleported(true, &tick, &context));
        assert_eq!(true, is_teleported(false, &tick, &context));

        tick.x = 1 as f32;
        tick.y = 1 as f32;
        tick.z = 1 as f32;
        assert_eq!(false, is_teleported(true, &tick, &context));
        assert_eq!(false, is_teleported(false, &tick, &context));

        tick.x = 15 as f32;
        assert_eq!(true, is_teleported(false, &tick, &context));

        tick.x = 15 as f32;
        tick.y = 15 as f32;
        assert_eq!(true, is_teleported(false, &tick, &context));

        tick.x = 1 as f32;
        tick.y = 15 as f32;
        assert_eq!(true, is_teleported(false, &tick, &context));

        tick.x = 1 as f32;
        tick.y = 1 as f32;
        tick.z = -100 as f32;
        assert_eq!(true, is_teleported(false, &tick, &context));
    }
}
