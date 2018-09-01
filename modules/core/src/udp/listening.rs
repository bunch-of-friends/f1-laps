use pipeline::input::Tick;
use schedule_recv;
use serialisation::{self, ReceivePacket};
use std::net::UdpSocket;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use storage;
use udp::Packet;

pub fn start_listening(port: i32, should_store_packets: bool, tx: mpsc::Sender<Tick>) {
    let socket = bind_to_address(format!("0.0.0.0:{}", port));
    let buffer_size = serialisation::get_buffer_size();

    let packets: Arc<Mutex<Vec<Packet>>> = Arc::new(Mutex::new(Vec::new()));

    if should_store_packets {
        let store_packets_tick = schedule_recv::periodic(Duration::from_secs(120));
        let packets_mutext_store = packets.clone();
        thread::spawn(move || loop {
            store_packets_tick.recv().unwrap();
            let mut packets_local = packets_mutext_store.lock().unwrap();

            if packets_local.len() > 0 {
                let packets_to_store = packets_local.clone();
                packets_local.clear();

                thread::spawn(move || {
                    storage::store_packets(packets_to_store);
                });
            }
        });
    }

    loop {
        let mut buffer = Vec::with_capacity(buffer_size);
        for _ in 0..buffer_size {
            buffer.push(0);
        }

        if let Some((amt, _src)) = socket.recv_from(&mut buffer).ok() {
            let tx = tx.clone();
            let packets_mutex_receive = packets.clone();
            let mut serialiser = serialisation::get_serialiser();
            thread::spawn(move || {
                if let Some(tick) = serialiser.converto_to_tick(&buffer, amt) {
                    tx.send(tick).ok();

                    if should_store_packets {
                        let mut packets_local = packets_mutex_receive.lock().unwrap();
                        packets_local.push(Packet::new(buffer));
                    }
                }
            });
        }
    }
}

fn bind_to_address(address: String) -> UdpSocket {
    return match UdpSocket::bind(&address) {
        Ok(socket) => socket,
        Err(e) => panic!("couldn't bind to: {}; e: {}", address, e),
    };
}