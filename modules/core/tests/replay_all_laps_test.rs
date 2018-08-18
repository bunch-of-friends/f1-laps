extern crate f1_laps_core;

mod common;

#[test]
fn replay_all_laps_test() {
    common::setup();

    let h = f1_laps_core::replay_all_laps_new();

    assert!(!h.0.join().is_err());
    assert!(!h.1.join().is_err());
}
