use context::*;
use pipeline::input::Tick;
use schedule_recv;
use serialisation::{self, ReceivePacket};
use std::net::UdpSocket;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use udp::Packet;

pub(crate) fn start_listening(context: &'static AppContext, port: i32, should_store_packets: bool, tx: &mpsc::Sender<Tick>) {
    let socket = bind_to_address(&format!("0.0.0.0:{}", port));
    context.log(LogEvent::UserInfo, &format!("listening on port {}", port));
    context.log(LogEvent::Debug, &format!("should_store_packets set to {}", should_store_packets));
    let buffer_size = serialisation::get_buffer_size();

    let packets: Arc<Mutex<Vec<Packet>>> = Arc::new(Mutex::new(Vec::new()));

    if should_store_packets {
        let store_packets_tick = schedule_recv::periodic(Duration::from_secs(120));
        let packets_mutext_store = packets.clone();
        thread::spawn(move || loop {
            store_packets_tick.recv().unwrap();
            let mut packets_local = packets_mutext_store.lock().unwrap();

            let len = packets_local.len();
            if len > 0 {
                context.log(LogEvent::Debug, &format!("checking for packets to store... storing packets, count {}", len));

                let packets_to_store = packets_local.clone();
                packets_local.clear();

                thread::spawn(move || {
                    context.storage.store_packets(&packets_to_store);
                });
            } else {
                context.log(LogEvent::Debug, "checking for packets to store... nothing to store");
            }
        });
    }

    let serialiser = Arc::new(Mutex::new(serialisation::get_serialiser()));

    loop {
        let mut buffer = Vec::with_capacity(buffer_size);
        for _ in 0..buffer_size {
            buffer.push(0);
        }

        match socket.recv_from(&mut buffer) {
            Ok((amt, _src)) => {
                let tx = tx.clone();
                let packets_mutex_receive = packets.clone();
                let serialiser_mutex = serialiser.clone();
                thread::spawn(move || {
                    let mut serialiser_local = serialiser_mutex.lock().unwrap();
                    if let Some(tick) = serialiser_local.converto_to_tick(&buffer, amt) {
                        tx.send(tick).ok();

                        if should_store_packets {
                            let mut packets_local = packets_mutex_receive.lock().unwrap();
                            packets_local.push(Packet::new(buffer));
                        }
                    }
                });
            }
            Err(_) => print!("error receiving packet"),
        }
    }
}

fn bind_to_address(address: &str) -> UdpSocket {
    match UdpSocket::bind(&address) {
        Ok(socket) => socket,
        Err(e) => panic!("couldn't bind to: {}; e: {}", address, e),
    }
}
