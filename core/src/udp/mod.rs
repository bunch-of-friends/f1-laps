pub mod packet;

use bincode;
use std::net::UdpSocket;
use std::string::String;
use std::sync::mpsc;
use std::thread;

use self::packet::Packet;
use aggregation::process_packet;
use aggregation::tick::Tick;

pub fn start_listening(port: i32, tx: mpsc::Sender<Tick>, should_store_packets: bool) {
    let socket = bind_to_address(format!("0.0.0.0:{}", port));

    let mut buf = [0; 1289]; //fixed sized for the f1 2017 game
    loop {
        let (amt, _src) = socket
            .recv_from(&mut buf)
            .expect("couldn't recieve a datagram");

        let tx = tx.clone();

        thread::spawn(move || {
            let packet = receive_packet(&mut buf[..amt]);

            // process the data
            let tick = process_packet(packet, should_store_packets);

            if tick.is_some() {
                tx.send(tick.unwrap())
                    .expect("failed to update the main thread")
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
