use pipeline::types::*;

pub fn build_labels(input_tick: &InputTick, context: &Context) -> PacketLabels {
    let session = Session::from_input_tick(&input_tick);
    let is_new_session = is_new_session(&session, context);
    let is_new_lap = is_new_lap(is_new_session, input_tick, context);
    let is_new_sector = is_new_sector(is_new_lap, input_tick, context);

    PacketLabels {
        is_new_session: is_new_session,
        is_new_lap: is_new_lap,
        is_new_sector: is_new_sector,
        session: session,
    }
}

fn is_new_session(session: &Session, context: &Context) -> bool {
    if let Some(ref previus_session) = context.session_context.session {
        !session.eq(previus_session)
    } else {
        true
    }
}

fn is_new_lap(is_new_session: bool, input_tick: &InputTick, context: &Context) -> bool {
    if is_new_session {
        true
    } else {
        input_tick.lap_number != context.session_context.lap_number
    }
}

fn is_new_sector(is_new_lap: bool, input_tick: &InputTick, context: &Context) -> bool {
    if is_new_lap {
        true
    } else {
        input_tick.sector != context.session_context.sector
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
                session_context: SessionContext::new(),
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

        context.session_context.sector = 0;
        tick.sector = 0;
        assert_eq!(false, is_new_sector(false, &tick, &context));

        context.session_context.sector = 0;
        tick.sector = 1;
        assert_eq!(true, is_new_sector(false, &tick, &context));
    }

    #[test]
    fn is_new_lap_test() {
        let i = create_input();
        let mut tick = i.0;
        let mut context = i.1;

        assert_eq!(true, is_new_lap(true, &tick, &context));

        context.session_context.lap_number = 1;
        tick.lap_number = 1;
        assert_eq!(false, is_new_lap(false, &tick, &context));

        context.session_context.lap_number = 1;
        tick.lap_number = 2;
        assert_eq!(true, is_new_lap(false, &tick, &context));
    }

    #[test]
    fn is_new_session_test() {
        let i = create_input();
        let mut session = Session::from_input_tick(&i.0);
        let mut context = i.1;

        context.session_context.session = None;
        assert_eq!(true, is_new_session(&session, &context));

        let mut s = session.clone();
        context.session_context.session = Some(s);
        assert_eq!(false, is_new_session(&session, &context));

        let mut s = session.clone();
        s.team_id = 10;
        context.session_context.session = Some(s);
        session.team_id = 9;
        assert_eq!(true, is_new_session(&session, &context));
    }

}
