use chrono::{DateTime, Utc};

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use pipeline::input::Tick;
use serialisation::ReceivePacket;
use storage;

pub fn replay_packets<T>(should_simulate_time: bool, serialiser: T, tx: mpsc::Sender<Tick>)
where
    T: ReceivePacket + 'static,
{
    let packets = storage::get_all_packets();
    let mut last_packet_time = Utc::now();
    for packet in &packets {
        if let Some(tick) = serialiser.converto_to_tick(&packet.bytes, packet.len()) {
            tx.send(tick).ok();

            if should_simulate_time {
                last_packet_time = simulate_time(last_packet_time, packet.timestamp);
            }
        }
    }
}

fn simulate_time(
    last_packet_time: DateTime<Utc>,
    current_packet_time: DateTime<Utc>,
) -> DateTime<Utc> {
    if current_packet_time < last_packet_time {
        //normally this is just the first packet only
        return current_packet_time;
    }

    let diff = current_packet_time - last_packet_time;
    if diff.num_milliseconds() > 1000 {
        //making big gaps max 1 second long
        thread::sleep(Duration::from_millis(1000))
    } else {
        thread::sleep(Duration::from_millis(diff.num_milliseconds() as u64));
    }

    current_packet_time
}
