// http://forums.codemasters.com/discussion/136948/f1-2018-udp-specification

/* 
ID | Packet Name      | bytes |
-------------------------------
0  | Motion           | 1341  |
1  | Session          | 147   |
2  | Lap Data         | 841   |
3  | Event            | 25    |
4  | Participants     | 1082  |
5  | Car setups       | 841   |
6  | Car telemetry    | 1085  |
7  | Status           | 1061  |
*/

use serialisation::ReceivePacket;

mod event;
mod header;
mod lap;
mod motion;
mod participants;
mod session;
mod setups;
mod status;
mod telemetry;

pub struct Serialiser {}

impl ReceivePacket for Serialiser {
    fn new() -> Self {
        Serialiser {}
    }

    fn get_buffer_size(&self) -> usize {
        // max possible size of 2018 packets
        1341
    }

    fn converto_to_tick(&self, datagram: &[u8], size: usize) -> Option<Tick> {
        None

        // bincode::deserialize::<Packet>(&content[..]).expect("failed to deserialise packet")
    }
}
