use pipeline::types::Tick;
use serialisation::ReceivePacket;

use std::net::UdpSocket;
use std::string::String;
use std::sync::mpsc;
use std::thread;

pub fn start_listening<T>(port: i32, serialiser: T, tx: mpsc::Sender<Tick>)
where
    T: ReceivePacket + 'static,
{
    let socket = bind_to_address(format!("0.0.0.0:{}", port));

    let mut buffer = Vec::with_capacity(serialiser.get_buffer_size());
    loop {
        if let Some((amt, _src)) = socket.recv_from(&mut buffer).ok() {
            let tx = tx.clone();
            thread::spawn(move || {
                if let Some(tick) = serialiser.converto_to_tick(&buffer, amt) {
                    tx.send(tick).ok();
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
