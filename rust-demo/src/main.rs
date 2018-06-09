extern crate f1_laps_core;

fn main() {
    println!("rust demo is running");

    f1_laps_core::initialise("../_data-storage".to_string());
    f1_laps_core::start_listening(20777, true);

    std::thread::park_timeout(std::time::Duration::from_secs(60));
}
