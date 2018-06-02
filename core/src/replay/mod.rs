use std::sync::mpsc;
use std::time::{Duration, Instant};
use thread;

use aggregation::process_packet;
use aggregation::tick::Tick;
use udp::packet::Packet;

pub fn stream_packets(tx: mpsc::Sender<Tick>, packets: Vec<Packet>) {
    println!("streaming stored packets");
    let packets_len = packets.len();

    let mut last_packet: Option<(Packet, Instant)> = None;
    for packet in packets {
        let tick = process_packet(packet, false);

        if tick.is_some() {
            // this whole block is here temporarily for some tests, then it will either go or get some love
            if last_packet.is_some() {
                let lp = last_packet.unwrap();

                let packet_diff = packet.time - lp.0.time;

                let mut packet_diff_ns = 0;
                if packet_diff > 0 as f32 {
                    packet_diff_ns = (packet_diff * 1000000000 as f32) as u32
                }

                let packet_diff_duration = Duration::new(0, packet_diff_ns);

                let since_last_send_duration = lp.1.elapsed();

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

            tx.send(tick.unwrap())
                .expect("failed to update the main thread");
            last_packet = Some((packet, Instant::now()));
        }
    }

    println!(
        "streaming stored packets finished, number of packets: {}",
        packets_len
    );
}
