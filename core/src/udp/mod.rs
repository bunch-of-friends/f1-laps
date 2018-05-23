use std::net::UdpSocket;
use std::string::String;
use std::thread;
use bincode;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

use models::api::Tick;
use models::packet::Packet;
use aggregation::process_packet;
use storage::replay::store_replay_data;

pub fn start_listening(port: i32, tx: mpsc::Sender<Tick>, store_packets: bool) {
    let socket = bind_to_address(format!("0.0.0.0:{}", port));

    // thread-safe vector for storing udp replays
    let replay_data = Arc::new(Mutex::new(Vec::<Packet>::new()));

    let mut buf = [0; 1289]; //fixed sized for the f1 2017 game
    loop {
        let (amt, _src) = socket
            .recv_from(&mut buf)
            .expect("couldn't recieve a datagram");

        // loop clones
        let replay_data = replay_data.clone();
        let tx = tx.clone();

        thread::spawn(move || {
            let packet = receive_packet(&mut buf[..amt]);

            // process the data
            let tick = process_packet(packet);

            if tick.is_some() {
                tx.send(tick.unwrap())
                    .expect("failed to update the main thread")
            }

            if !store_packets {
                return;
            }

            // update received udp list collection
            let mut unlocked_replay_data = replay_data.lock().unwrap();
            unlocked_replay_data.push(packet);

            // every 500 packets (roughtly a lap), store the data
            let len = unlocked_replay_data.len();
            if len == 500 {
                let results_copy = unlocked_replay_data.to_vec();
                unlocked_replay_data.truncate(0);

                thread::spawn(move || {
                    store_replay_data(results_copy);
                });
            }
        });
    }
}

fn bind_to_address(address: String) -> UdpSocket {
    return match UdpSocket::bind(&address) {
        Ok(socket) => {
            println!("listening on: {} ", address);
            socket
        }
        Err(e) => panic!("couldn't bind to: {}; e: {}", address, e),
    };
}

fn receive_packet(content: &[u8]) -> Packet {
    let deserialised: Packet =
        bincode::deserialize(&content[..]).expect("failed to deserialise packet");
    return deserialised;
}
