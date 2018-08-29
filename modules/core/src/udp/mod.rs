use pipeline::input::Tick;
use serialisation::ReceivePacket;
use storage;

use schedule_recv;

use std::net::UdpSocket;
use std::string::String;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_listening<T>(port: i32, serialiser: T, tx: mpsc::Sender<Tick>)
where
    T: ReceivePacket + 'static,
{
    let socket = bind_to_address(format!("0.0.0.0:{}", port));
    let buffer_size = serialiser.get_buffer_size();

    let packets: Arc<Mutex<Vec<Vec<u8>>>> = Arc::new(Mutex::new(Vec::new()));
    let store_packets_tick = schedule_recv::periodic(Duration::from_secs(120));

    let packets_mutext_store = packets.clone();
    thread::spawn(move || loop {
        store_packets_tick.recv().unwrap();
        println!("checking if there is someting to store...");
        let mut packets_local = packets_mutext_store.lock().unwrap();

        if packets_local.len() > 0 {
            println!("store");
            let packets_to_store = packets_local.clone();
            storage::store_packets(packets_to_store);

            packets_local.clear();
        } else {
            println!("nothing to store");
        }
    });

    loop {
        let mut buffer = Vec::with_capacity(buffer_size);
        for _ in 0..buffer_size {
            buffer.push(0);
        }

        if let Some((amt, _src)) = socket.recv_from(&mut buffer).ok() {
            let tx = tx.clone();
            let serialiser = serialiser.clone();
            let packets_mutex_receive = packets.clone();
            thread::spawn(move || {
                if let Some(tick) = serialiser.converto_to_tick(&buffer, amt) {
                    tx.send(tick).ok();
                    let mut packets_local = packets_mutex_receive.lock().unwrap();
                    packets_local.push(buffer);
                }
            });
        }
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
