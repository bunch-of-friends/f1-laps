use chrono::{DateTime, Utc};

use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

use context::{AppContext, LogEvent};
use pipeline::input::Tick;
use serialisation::{self, ReceivePacket};
use udp::Packet;

pub(crate) fn replay_packets(context: &'static AppContext, should_simulate_time: bool, tx: mpsc::Sender<Tick>) {
    context.log(LogEvent::UserInfo, "replaying stored packets");

    let (packet_tx, packet_rx): (mpsc::Sender<Vec<Packet>>, mpsc::Receiver<Vec<Packet>>) = mpsc::channel();

    thread::spawn(move || {
        context.storage.get_all_packets(&packet_tx);
    });

    let mut last_packet_time = Utc::now();
    let mut serialiser = serialisation::get_serialiser();

    thread::spawn(move || loop {
        match packet_rx.try_recv() {
            Ok(packets) => {
                context.log(LogEvent::Debug, &format!("packets receeived, count: {}", packets.len()));
                for packet in &packets {
                    if let Some(tick) = serialiser.converto_to_tick(context, &packet.bytes, packet.len()) {
                        tx.send(tick).ok();

                        if should_simulate_time {
                            last_packet_time = simulate_time(last_packet_time, packet.timestamp);
                        }
                    }
                }
            }
            Err(TryRecvError::Disconnected) => {
                context.log(LogEvent::UserInfo, "replaying packets finished");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    });
}

fn simulate_time(last_packet_time: DateTime<Utc>, current_packet_time: DateTime<Utc>) -> DateTime<Utc> {
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
