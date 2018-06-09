extern crate f1_laps_core;

fn main() {
    println!("rust demo is running");

    f1_laps_core::initialise("../_data-storage".to_string());

    f1_laps_core::replay_all_laps();
    // f1_laps_core::start_listening(20777, true);

    loop {
        if let Some(tick) = f1_laps_core::get_next_tick() {
            println!(">> {:?}", tick.live_data.current_speed);
        }
        std::thread::park_timeout(std::time::Duration::from_millis(30));
    }
}
