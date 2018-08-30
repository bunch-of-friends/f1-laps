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

        Output {
            labels: labels,
            events: events,
            session_data: tick.session_data,
            lap_data: tick.lap_data,
            car_status: tick.car_status,
            car_telemetry: tick.car_telemetry,
            car_motion: tick.car_motion,
        }
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
