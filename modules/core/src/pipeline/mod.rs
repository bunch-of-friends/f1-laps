pub(crate) mod input;
pub(crate) mod output;
mod routines;

use self::input::*;
use self::output::*;
use pipeline::routines::lap_telemetry_store::LapTelemetryTempStore;

pub struct Pipeline {
    context: Context,
    current_lap_telemetry: LapTelemetryTempStore,
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            context: Context::empty(),
            current_lap_telemetry: LapTelemetryTempStore::new(),
        }
    }

    pub fn process(&mut self, tick: Tick) -> Output {
        let labels = routines::labels::build_labels(&tick, &self.context);
        let events = routines::events::build_events(&tick, &self.context, &labels);

        // let finished_lap_ticks = routines::lap_ticks::update_lap_ticks(
        //     tick,
        //     &labels,
        //     &events,
        //     &mut self.current_lap_telemetry,
        // );

        // if self.should_store_laps {
        //     self.try_store_lap(finished_lap_ticks, &events);
        // }

        let new_context = routines::context::build_context(&tick, &self.context);

        self.context = new_context;

        convert_to_output(tick, labels, events)
    }

    // fn try_store_lap(&self, finished_lap_ticks: Option<Vec<CarTelemetry>>, events: &Events) {
    //     if let Some(ticks) = finished_lap_ticks {
    //         if let Some(ref finished_lap) = events.finished_lap {
    //             let metadata = LapMetadata::new(
    //                 &self.context.session_context.session,
    //                 finished_lap,
    //                 self.context.session_context.car_status.tyre_compound,
    //             );

    //             let t = thread::spawn(move || {
    //                 storage::store_lap(ticks, &metadata);
    //             });

    //             if self.should_wait_for_fs {
    //                 let j = t.join();
    //                 assert!(!j.is_err());
    //             }
    //         }
    //     }
    // }
}

fn convert_to_output(tick: Tick, labels: Labels, events: Events) -> Output {
    Output {
        labels: labels,
        events: events,
        session_data: tick.session_data,
        lap_data: convert_to_opt_multi_car(tick.lap_data),
        car_status: convert_to_opt_multi_car_opt(tick.car_status),
        car_telemetry: convert_to_opt_multi_car(tick.car_telemetry),
        car_motion: convert_to_opt_multi_car(tick.car_motion),
        car_setup: convert_to_opt_multi_car_opt(tick.car_setup),
        participants_info: convert_to_opt_multi_car_opt(tick.participants_info),
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

