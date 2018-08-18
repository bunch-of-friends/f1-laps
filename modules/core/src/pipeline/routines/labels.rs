use pipeline::types::*;

pub fn build_labels(input_tick: &InputTick, context: &Context) -> Labels {
    let session = Session::from_input_tick(&input_tick);
    let is_new_session = is_new_session(&session, context);
    let is_new_lap = is_new_lap(is_new_session, input_tick, context);
    let is_new_sector = is_new_sector(is_new_lap, input_tick, context);
    let is_flashback = is_flashback(is_new_session, &session, context);

    Labels {
        is_new_session: is_new_session,
        is_new_lap: is_new_lap,
        is_new_sector: is_new_sector,
        is_flashback: is_flashback,
        is_teleported: false,
        current_session: session,
        current_lap: Lap::from_input_tick(&input_tick),
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

fn is_flashback(is_new_session: bool, session: &Session, context: &Context) -> bool {
    if is_new_session {
        false
    } else {
        context.session_context.session.session_time > session.session_time
    }
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
        let mut session = Session::from_input_tick(&i.0);
        let mut context = i.1;

        context.session_context.session.session_time = 100.12;
        session.session_time = 150.15;
        assert_eq!(false, is_flashback(false, &session, &context));
        assert_eq!(false, is_flashback(true, &session, &context));

        context.session_context.session.session_time = 100.12;
        session.session_time = 50.15;
        assert_eq!(true, is_flashback(false, &session, &context));
        assert_eq!(false, is_flashback(true, &session, &context));
    }
}
