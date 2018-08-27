#![allow(non_snake_case)]

use pipeline::input::*;
use serialisation::ReceivePacket;

use bincode;

mod conversion;
pub(crate) mod packets;

static HEADER_SIZE: usize = 168;
static PACKET_MAX_SIZE: usize = 1341;

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

#[derive(Clone)]
pub struct Serialiser {}

impl Serialiser {
    fn append_body(&self, tick: &mut Tick, header: packets::PacketHeader, datagram: &[u8]) {
        match header.m_packetId {
            0 => {
                println!("motion received");
                if let Some(motion) = self.serialise_motion(datagram) {
                    tick.car_motion = Some(motion.to_model(&header));
                }
            }
            1 => {
                println!("session received");
                if let Some(session) = self.serialise_session(datagram) {
                    tick.session_data = Some(session.to_model());
                }
            }
            2 => {
                println!("lap data received");
                if let Some(lap_data) = self.serialise_lap_data(datagram) {
                    tick.lap_data = Some(lap_data.to_model(&header));
                }
            }
            3 => {
                println!("events received");
                let events = self.serialise_events(datagram);
                // nothing for now
            }
            4 => {
                println!("participants received");
                let participants = self.serialise_participants(datagram);
                // nothing for now
            }
            5 => {
                println!("setups received");
                let setups = self.serialise_setups(datagram);
                // nothing for now
            }
            6 => {
                println!("telemetry received");
                if let Some(telemetry) = self.serialise_telemetry(datagram) {
                    tick.car_telemetry = Some(telemetry.to_model(&header));
                }
            }
            7 => {
                println!("status received");
                if let Some(status) = self.serialise_status(datagram) {
                    tick.car_status = Some(status.to_model(&header));
                }
            }
            _ => {
                println!("unknown packet received");
            }
        }
    }

    fn serialise_header(&self, datagram: &[u8]) -> Option<packets::PacketHeader> {
        bincode::deserialize::<packets::PacketHeader>(&datagram[0..HEADER_SIZE]).ok()
    }

    fn serialise_motion(&self, datagram: &[u8]) -> Option<packets::PacketMotionData> {
        bincode::deserialize::<packets::PacketMotionData>(&datagram[HEADER_SIZE..]).ok()
    }

    fn serialise_session(&self, datagram: &[u8]) -> Option<packets::PacketSessionData> {
        bincode::deserialize::<packets::PacketSessionData>(&datagram[HEADER_SIZE..]).ok()
    }

    fn serialise_lap_data(&self, datagram: &[u8]) -> Option<packets::PacketLapData> {
        bincode::deserialize::<packets::PacketLapData>(&datagram[HEADER_SIZE..]).ok()
    }

    fn serialise_events(&self, datagram: &[u8]) -> Option<packets::PacketEventData> {
        bincode::deserialize::<packets::PacketEventData>(&datagram[HEADER_SIZE..]).ok()
    }

    fn serialise_participants(&self, datagram: &[u8]) -> Option<packets::PacketParticipantsData> {
        bincode::deserialize::<packets::PacketParticipantsData>(&datagram[HEADER_SIZE..]).ok()
    }

    fn serialise_setups(&self, datagram: &[u8]) -> Option<packets::PacketCarSetupData> {
        bincode::deserialize::<packets::PacketCarSetupData>(&datagram[HEADER_SIZE..]).ok()
    }

    fn serialise_telemetry(&self, datagram: &[u8]) -> Option<packets::PacketCarTelemetryData> {
        bincode::deserialize::<packets::PacketCarTelemetryData>(&datagram[HEADER_SIZE..]).ok()
    }

    fn serialise_status(&self, datagram: &[u8]) -> Option<packets::PacketCarStatusData> {
        bincode::deserialize::<packets::PacketCarStatusData>(&datagram[HEADER_SIZE..]).ok()
    }
}

impl ReceivePacket for Serialiser {
    fn new() -> Self {
        Serialiser {}
    }

    fn get_buffer_size(&self) -> usize {
        PACKET_MAX_SIZE
    }

    fn converto_to_tick(&self, datagram: &[u8], size: usize) -> Option<Tick> {
        if let Some(header_raw) = self.serialise_header(datagram) {
            let mut tick = Tick::new(header_raw.to_model());

            self.append_body(&mut tick, header_raw, datagram);

            Some(tick)
        } else {
            println!("failed to deserialise header, datagram size: {}", size);
            None
        }
    }
}
