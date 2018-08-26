use std::sync::mpsc;
use std::time::{Duration, Instant};
use thread;

use pipeline::types::Tick;

pub fn stream<T>(tx: mpsc::Sender<Tick>, streamables: Vec<T>, shoud_simulate_time: bool)
where
    T: Streamable,
{
    let mut last_iter: Option<(Tick, Instant)> = None;
    for s in streamables {
        let tick = s.get_tick();

        // this whole block is here temporarily for some tests, then it will either go or get some love
        if shoud_simulate_time && last_iter.is_some() {
            let last_iter_unwp = last_iter.unwrap();

            let time_diff = tick.session_time - last_iter_unwp.0.session_time;

            let mut time_diff_ns = 0;
            if time_diff > 0 as f32 {
                time_diff_ns = (time_diff * 1000000000 as f32) as u32
            }

            let packet_diff_duration = Duration::new(0, time_diff_ns);

            let since_last_send_duration = last_iter_unwp.1.elapsed();

            let mut sleep_needed = packet_diff_duration;
            if packet_diff_duration > since_last_send_duration {
                sleep_needed = packet_diff_duration - since_last_send_duration;
            }

            if sleep_needed.as_secs() > 0 {
                thread::sleep(Duration::from_secs(10));
            } else {
                let min_sleep = Duration::from_millis(20);
                if sleep_needed > min_sleep {
                    let applied = sleep_needed - Duration::new(0, 2160000); // totally based on observation only, no science here
                    thread::sleep(applied);
                }
            }
        }

        tx.send(tick.clone()).ok();
        last_iter = Some((tick, Instant::now()));
    }
}

pub trait Streamable {
    fn get_tick(&self) -> Tick;
}

impl Streamable for Tick {
    fn get_tick(&self) -> Tick {
        self.clone()
    }
}
