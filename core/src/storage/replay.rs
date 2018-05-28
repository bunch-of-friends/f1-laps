use std::fs::{read_dir, File};

use std::sync::mpsc;
use std::time::{Duration, Instant};
use thread;

use bincode;
use chrono::Local;

use aggregation::tick::Tick;
use udp::packet::Packet;
use aggregation::process_packet;

pub fn store_replay_data(packets: Vec<Packet>) {
    let date = Local::now();
    let path = format!(
        "storage/test_storage_{}.bin",
        date.format("%Y-%m-%d-%H-%M-%S-%f")
    );
    println!("path {}", path);
    let file = File::create(path).unwrap();
    bincode::serialize_into(file, &packets).unwrap();
}

pub fn get_replay_data(tx: mpsc::Sender<Tick>) {
    let paths = read_dir("storage/").unwrap();

    let mut file_paths: Vec<String> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if file_name.ends_with(".bin") && !file_name.ends_with("records.bin") {
            file_paths.push(file_name.to_owned());
        }
    }

    file_paths.sort();

    let mut packets = Vec::<Packet>::new();
    for path in file_paths {
        let full_path = format!("storage/{}", path);
        println!("loading file >> {}", full_path);

        let file = File::open(full_path).expect("failed to open file");
        let data = bincode::deserialize_from::<File, Vec<Packet>>(file).ok();

        if data.is_some() {
            packets.extend(data.unwrap());
        }
    }

    println!("streaming stored packets");

    let mut last_packet: Option<(Packet, Instant)> = None;
    for packet in packets {
        let tick = process_packet(packet);

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

            last_packet = Some((packet, Instant::now()));
            tx.send(tick.unwrap())
                .expect("failed to update the main thread")
        }
    }
}
