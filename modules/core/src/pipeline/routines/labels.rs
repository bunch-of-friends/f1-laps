use pipeline::input::*;
use pipeline::output::*;

pub fn build_labels(tick: &Tick, context: &Context) -> Labels {
    let is_new_session = is_new_session(&tick, context);

    let is_new_lap = is_new_lap(is_new_session, tick, context);
    let is_new_sector = is_new_sector(is_new_lap, tick, context);
    let is_flashback = is_flashback(is_new_session, is_new_lap, tick, context);
    let is_teleported = is_teleported(is_new_session, tick, context);

    Labels {
        is_new_session: is_new_session,
        is_new_lap: is_new_lap,
        is_new_sector: is_new_sector,
        is_flashback: is_flashback,
        is_teleported: is_teleported,
    }
}

fn is_new_session(tick: &Tick, context: &Context) -> bool {
    if let Some(ref header) = context.session_context.header {
        tick.header.session_uid != header.session_uid
    } else {
        true
    }
}

fn is_new_lap(is_new_session: bool, tick: &Tick, context: &Context) -> bool {
    if is_new_session {
        return true;
    }

    if let Some(ref current_lap) = context.session_context.lap {
        tick.lap_data.current_lap_number != current_lap.lap_number
    } else {
        true
    }
}

fn is_new_sector(is_new_lap: bool, tick: &Tick, context: &Context) -> bool {
    if is_new_lap {
        return true;
    }

    if let Some(ref current_sector) = context.session_context.sector {
        tick.lap_data.current_sector_number != current_sector.sector_number
    } else {
        false
    }
}

fn is_flashback(is_new_session: bool, is_new_lap: bool, tick: &Tick, context: &Context) -> bool {
    if is_new_session || is_new_lap {
        return false;
    }

    if let Some(ref current_lap) = context.session_context.lap {
        tick.lap_data.current_lap_time < current_lap.lap_time
    } else {
        false
    }
}

fn is_teleported(is_new_session: bool, tick: &Tick, context: &Context) -> bool {
    if is_new_session {
        return false;
    }

    if let Some(ref current_motion) = context.session_context.car_motion {
        let x_diff = (tick.car_motion.x - current_motion.x).abs();
        let y_diff = (tick.car_motion.y - current_motion.y).abs();
        let z_diff = (tick.car_motion.z - current_motion.z).abs();

        let max = 10 as f32;

        x_diff > max || y_diff > max || z_diff > max
    } else {
        false
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test_utils;

//     fn create_input() -> (Tick, Context) {
//         (
//             test_utils::create_tick(),
//             Context {
//                 session_context: SessionContext::empty(),
//                 history_context: HistoryContext {},
//             },
//         )
//     }

//     #[test]
//     fn is_new_sector_test() {
//         let i = create_input();
//         let mut tick = i.0;
//         let mut context = i.1;

//         assert_eq!(true, is_new_sector(true, &tick, &context));

//         context.session_context.sector.sector_number = 0;
//         tick.sector_number = 0;
//         assert_eq!(false, is_new_sector(false, &tick, &context));

//         context.session_context.sector.sector_number = 0;
//         tick.sector_number = 1;
//         assert_eq!(true, is_new_sector(false, &tick, &context));
//     }

//     #[test]
//     fn is_new_lap_test() {
//         let i = create_input();
//         let mut tick = i.0;
//         let mut context = i.1;

//         assert_eq!(true, is_new_lap(true, &tick, &context));

//         context.session_context.lap.lap_number = 1;
//         tick.lap_number = 1;
//         assert_eq!(false, is_new_lap(false, &tick, &context));

//         context.session_context.lap.lap_number = 1;
//         tick.lap_number = 2;
//         assert_eq!(true, is_new_lap(false, &tick, &context));
//     }

//     #[test]
//     fn is_new_session_test() {
//         let i = create_input();
//         let mut session = Session::from_tick(&i.0);
//         let mut context = i.1;

//         context.session_context = SessionContext::empty();
//         assert_eq!(true, is_new_session(&session, &context));

//         context.session_context.session = session.clone();
//         assert_eq!(false, is_new_session(&session, &context));

//         let mut s = session.clone();
//         s.team_id = 10;
//         context.session_context.session = s;
//         session.team_id = 9;
//         assert_eq!(true, is_new_session(&session, &context));
//     }

//     #[test]
//     fn is_flashback_test() {
//         let i = create_input();
//         let mut lap = Lap::from_tick(&i.0);
//         let mut context = i.1;

//         context.session_context.lap.lap_time = 100.12;
//         lap.lap_time = 150.15;
//         assert_eq!(false, is_flashback(false, false, &lap, &context));
//         assert_eq!(false, is_flashback(true, true, &lap, &context));
//         assert_eq!(false, is_flashback(false, true, &lap, &context));
//         assert_eq!(false, is_flashback(true, false, &lap, &context));

//         context.session_context.lap.lap_time = 100.12;
//         lap.lap_time = 50.15;
//         assert_eq!(true, is_flashback(false, false, &lap, &context));
//         assert_eq!(false, is_flashback(true, true, &lap, &context));
//         assert_eq!(false, is_flashback(false, true, &lap, &context));
//         assert_eq!(false, is_flashback(true, false, &lap, &context));
//     }

//     #[test]
//     fn is_teleported_test() {
//         let i = create_input();
//         let mut tick = i.0;
//         let mut context = i.1;

//         context.session_context.position.x = 1 as f32;
//         context.session_context.position.y = 1 as f32;
//         context.session_context.position.z = 1 as f32;
//         tick.x = 500 as f32;
//         tick.y = 500 as f32;
//         tick.z = 500 as f32;
//         assert_eq!(false, is_teleported(true, &tick, &context));
//         assert_eq!(true, is_teleported(false, &tick, &context));

//         tick.x = 1 as f32;
//         tick.y = 1 as f32;
//         tick.z = 1 as f32;
//         assert_eq!(false, is_teleported(true, &tick, &context));
//         assert_eq!(false, is_teleported(false, &tick, &context));

//         tick.x = 15 as f32;
//         assert_eq!(true, is_teleported(false, &tick, &context));

//         tick.x = 15 as f32;
//         tick.y = 15 as f32;
//         assert_eq!(true, is_teleported(false, &tick, &context));

//         tick.x = 1 as f32;
//         tick.y = 15 as f32;
//         assert_eq!(true, is_teleported(false, &tick, &context));

//         tick.x = 1 as f32;
//         tick.y = 1 as f32;
//         tick.z = -100 as f32;
//         assert_eq!(true, is_teleported(false, &tick, &context));
//     }
// }
