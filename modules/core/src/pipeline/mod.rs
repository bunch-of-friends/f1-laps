pub(crate) mod input;
pub(crate) mod output;
mod routines;

use self::input::*;
use self::output::*;

pub struct Pipeline {
    context: Context,
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            context: Context::empty(),
        }
    }

    pub fn process(&mut self, tick: Tick) -> Output {
        let labels = routines::labels::build_labels(&tick, &self.context);
        let events = routines::events::build_events(&tick, &self.context, &labels);

        let new_context = routines::context::build_context(&tick, &self.context);

        self.context = new_context;

        convert_to_output(tick, labels, events)
    }
}

fn convert_to_output(tick: Tick, labels: Labels, events: Events) -> Output {
    let lap_data = tick.lap_data;
    let car_status = tick.car_status;
    let car_telemetry = tick.car_telemetry;
    let car_motion = tick.car_motion;
    let participants_info = tick.participants_info;

    Output {
        labels: labels,
        events: events,
        session_data: tick.session_data,
        lap_data: convert_to_opt_multi_car(lap_data),
        car_status: convert_to_opt_multi_car_opt(car_status),
        car_telemetry: convert_to_opt_multi_car(car_telemetry),
        car_motion: convert_to_opt_multi_car(car_motion),
        participants_info: convert_to_opt_multi_car_opt(participants_info),
    }
}

fn convert_to_opt_multi_car<T>(multi_car: MultiCarData<T>) -> OptMultiCarData<T>
where
    T: Clone,
{
    OptMultiCarData {
        player: multi_car.player,
        others: Some(multi_car.others),
    }
}

fn convert_to_opt_multi_car_opt<T>(multi_car: Option<MultiCarData<T>>) -> Option<OptMultiCarData<T>>
where
    T: Clone,
{
    if let Some(mc) = multi_car {
        Some(OptMultiCarData {
            player: mc.player,
            others: Some(mc.others),
        })
    } else {
        None
    }
}
