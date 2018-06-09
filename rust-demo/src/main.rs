extern crate f1_laps_core;

fn main() {
    println!("rust demo is running");

    f1_laps_core::initialise("../_data-storage".to_string());

    let metadata = f1_laps_core::get_all_laps_metadata();
    println!("metadata >> {:?}", metadata);

    let lap_data = f1_laps_core::get_lap_data(metadata[0].identifier.clone());
    println!("lap data len >> {:?}", lap_data.len());

    f1_laps_core::replay_lap(metadata[0].identifier.clone());

    // f1_laps_core::replay_all_laps();
    // f1_laps_core::start_listening(20777, true);

    subscribe_to_ticks();
}

fn subscribe_to_ticks() {
    loop {
        if let Some(tick) = f1_laps_core::get_next_tick() {
            println!("tick >> {:?}", tick.live_data.current_speed);
        }
        std::thread::park_timeout(std::time::Duration::from_millis(30));
    }
}
