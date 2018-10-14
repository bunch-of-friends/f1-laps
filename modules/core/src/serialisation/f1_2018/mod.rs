#![allow(non_snake_case)]

mod conversion;
mod packets;
mod serialiser;
mod id_mapper;

use serialisation::{MapId, ReceivePacket};

// http://forums.codemasters.com/discussion/136948/f1-2018-udp-specification

/* 
ID | Packet Name      | bytes | frequency
-----------------------------------------------------
0  | Motion           | 1341  | every frame (menu settings, 20-60Hz)
1  | Session          | 147   | 2Hz
2  | Lap Data         | 841   | every frame (menu settings, 20-60Hz)
3  | Event            | 25    | on event
4  | Participants     | 1082  | every 5 seconds
5  | Car setups       | 841   | 2Hz
6  | Car telemetry    | 1085  | every frame (menu settings, 20-60Hz)
7  | Status           | 1061  | 2Hz
*/

pub(crate) fn get_buffer_size() -> usize {
    serialiser::PACKET_MAX_SIZE
}

pub(crate) fn get_serialiser() -> impl ReceivePacket {
    serialiser::Serialiser::new()
}

pub(crate) fn get_id_mapper() -> impl MapId {
    id_mapper::IdMapper::new()
}
