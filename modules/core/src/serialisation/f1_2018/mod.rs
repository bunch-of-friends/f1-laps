#![allow(non_snake_case)]

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

use pipeline::types::*;
use serialisation::ReceivePacket;

use bincode;

mod conversion;
pub(crate) mod packets;

#[derive(Clone)]
pub struct Serialiser {}

impl Serialiser {
    fn serialise_header(&self, datagram: &[u8]) -> Option<packets::PacketHeader> {
        bincode::deserialize::<packets::PacketHeader>(&datagram[0..168]).ok()
    }

    fn serialise_motion(&self, datagram: &[u8]) -> Option<packets::PacketMotionData> {
        bincode::deserialize::<packets::PacketMotionData>(&datagram[..]).ok()
    }

    fn serialise_session(&self, datagram: &[u8]) -> Option<packets::PacketSessionData> {
        bincode::deserialize::<packets::PacketSessionData>(&datagram[..]).ok()
    }

    fn serialise_lap_data(&self, datagram: &[u8]) -> Option<packets::PacketLapData> {
        bincode::deserialize::<packets::PacketLapData>(&datagram[..]).ok()
    }

    fn serialise_events(&self, datagram: &[u8]) -> Option<packets::PacketEventData> {
        bincode::deserialize::<packets::PacketEventData>(&datagram[..]).ok()
    }

    fn serialise_participants(&self, datagram: &[u8]) -> Option<packets::PacketParticipantsData> {
        bincode::deserialize::<packets::PacketParticipantsData>(&datagram[..]).ok()
    }

    fn serialise_setups(&self, datagram: &[u8]) -> Option<packets::PacketCarSetupData> {
        bincode::deserialize::<packets::PacketCarSetupData>(&datagram[..]).ok()
    }

    fn serialise_telemetry(&self, datagram: &[u8]) -> Option<packets::PacketCarTelemetryData> {
        bincode::deserialize::<packets::PacketCarTelemetryData>(&datagram[..]).ok()
    }

    fn serialise_status(&self, datagram: &[u8]) -> Option<packets::PacketCarStatusData> {
        bincode::deserialize::<packets::PacketCarStatusData>(&datagram[..]).ok()
    }
}

impl ReceivePacket for Serialiser {
    fn new() -> Self {
        Serialiser {}
    }

    fn get_buffer_size(&self) -> usize {
        // max possible size of 2018 packets
        1341
    }

    fn converto_to_tick(&self, datagram: &[u8], size: usize) -> Option<Tick> {
        if let Some(header_raw) = self.serialise_header(datagram) {
            let mut tick = Tick::new(header_raw.to_model());

            match header_raw.m_packetId {
                0 => {
                    println!("motion received");
                    let motion = self.serialise_motion(datagram);
                }
                1 => {
                    println!("session received");
                    let session = self.serialise_session(datagram);
                }
                2 => {
                    println!("lap data received");
                    let lap_data = self.serialise_lap_data(datagram);
                }
                3 => {
                    println!("events received");
                    let events = self.serialise_events(datagram);
                }
                4 => {
                    println!("participants received");
                    let participants = self.serialise_participants(datagram);
                }
                5 => {
                    println!("setups received");
                    let setups = self.serialise_setups(datagram);
                }
                6 => {
                    println!("telemetry received");
                    let telemetry = self.serialise_telemetry(datagram);
                }
                7 => {
                    println!("status received");
                    let status = self.serialise_status(datagram);
                }
                _ => {
                    println!("unknown packet received");
                    return None;
                }
            }

            Some(tick)
        } else {
            println!("failed to deserialise header, datagram size: {}", size);
            None
        }
    }
}
