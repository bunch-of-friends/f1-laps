pub mod packet;

use pipeline::types::Tick;

use bincode;
use std::net::UdpSocket;
use std::string::String;
use std::sync::mpsc;
use std::thread;

use self::packet::Packet;

pub fn start_listening(port: i32, tx: mpsc::Sender<Tick>) {
    let socket = bind_to_address(format!("0.0.0.0:{}", port));

    let mut buf = [0; 1341]; // max size of F1 2018 packet
    loop {
        if let Some((amt, _src)) = socket.recv_from(&mut buf).ok() {
            let tx = tx.clone();
            thread::spawn(move || {
                let packet = receive_packet(&mut buf[..amt]);
                let tick = Tick::from_packet(&packet);
                tx.send(tick).ok();
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

fn receive_packet(content: &[u8]) -> Packet {
    bincode::deserialize::<Packet>(&content[..]).expect("failed to deserialise packet")
}
