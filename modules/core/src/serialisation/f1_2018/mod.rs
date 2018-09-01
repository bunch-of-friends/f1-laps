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
ID | Packet Name      | bytes | present in each frame
-----------------------------------------------------
0  | Motion           | 1341  | true
1  | Session          | 147   | false
2  | Lap Data         | 841   | true
3  | Event            | 25    | false
4  | Participants     | 1082  | false
5  | Car setups       | 841   | false
6  | Car telemetry    | 1085  | true
7  | Status           | 1061  | false
*/

pub(crate) fn get_buffer_size() -> usize {
    PACKET_MAX_SIZE
}

#[derive(Clone)]
pub(crate) struct Serialiser {
    pub current_frame: Option<Frame>,
}

impl Serialiser {
    fn append_to_current_frame(&mut self, header: &packets::PacketHeader, datagram: &[u8]) {
        if let Some(frame) = self.current_frame.as_mut() {
            match header.m_packetId {
                0 => {
                    if let Some(motion) = serialise_motion(datagram) {
                        frame.car_motion = Some(motion.to_model(&header));
                    }
                }
                1 => {
                    if let Some(session) = serialise_session(datagram) {
                        frame.session_data = Some(session.to_model());
                    }
                }
                2 => {
                    if let Some(lap_data) = serialise_lap_data(datagram) {
                        frame.lap_data = Some(lap_data.to_model(&header));
                    }
                }
                3 => {
                    let _events = serialise_events(datagram);
                    // nothing for now
                }
                4 => {
                    let _participants = serialise_participants(datagram);
                    // nothing for now
                }
                5 => {
                    let _setups = serialise_setups(datagram);
                    // nothing for now
                }
                6 => {
                    if let Some(telemetry) = serialise_telemetry(datagram) {
                        frame.car_telemetry = Some(telemetry.to_model(&header));
                    }
                }
                7 => {
                    if let Some(status) = serialise_status(datagram) {
                        frame.car_status = Some(status.to_model(&header));
                    }
                }
                _ => {
                    println!("unknown packet received");
                }
            }
        } else {
            panic!("appending to current frame, which is None");
        }
    }

    fn is_current_frame(&self, header: &packets::PacketHeader) -> bool {
        if let Some(ref current_frame) = self.current_frame {
            current_frame.header.session_uid == header.m_sessionUID
                && current_frame.header.frame_id == header.m_frameIdentifier
        } else {
            false
        }
    }
}

impl ReceivePacket for Serialiser {
    fn new() -> Self {
        Serialiser {
            current_frame: None,
        }
    }

    fn converto_to_tick(&mut self, datagram: &[u8], _size: usize) -> Option<Tick> {
        let header = serialise_header(datagram)?;

        let mut result: Option<Tick> = None;

        if !self.is_current_frame(&header) {
            if self.current_frame.is_some() {
                let previous_frame = self.current_frame.clone().unwrap();
                self.current_frame = Some(Frame::new(&header));
                result = Some(previous_frame.to_tick())
            } else {
                self.current_frame = Some(Frame::new(&header));
            }
        }
        self.append_to_current_frame(&header, datagram);

        result
    }
}

fn serialise_header(datagram: &[u8]) -> Option<packets::PacketHeader> {
    bincode::deserialize::<packets::PacketHeader>(&datagram[0..HEADER_SIZE]).ok()
}

fn serialise_motion(datagram: &[u8]) -> Option<packets::PacketMotionData> {
    bincode::deserialize::<packets::PacketMotionData>(&datagram[..]).ok()
}

fn serialise_session(datagram: &[u8]) -> Option<packets::PacketSessionData> {
    bincode::deserialize::<packets::PacketSessionData>(&datagram[..]).ok()
}

fn serialise_lap_data(datagram: &[u8]) -> Option<packets::PacketLapData> {
    bincode::deserialize::<packets::PacketLapData>(&datagram[..]).ok()
}

fn serialise_events(datagram: &[u8]) -> Option<packets::PacketEventData> {
    bincode::deserialize::<packets::PacketEventData>(&datagram[..]).ok()
}

fn serialise_participants(datagram: &[u8]) -> Option<packets::PacketParticipantsData> {
    bincode::deserialize::<packets::PacketParticipantsData>(&datagram[..]).ok()
}

fn serialise_setups(datagram: &[u8]) -> Option<packets::PacketCarSetupData> {
    bincode::deserialize::<packets::PacketCarSetupData>(&datagram[..]).ok()
}

fn serialise_telemetry(datagram: &[u8]) -> Option<packets::PacketCarTelemetryData> {
    bincode::deserialize::<packets::PacketCarTelemetryData>(datagram).ok()
}

fn serialise_status(datagram: &[u8]) -> Option<packets::PacketCarStatusData> {
    bincode::deserialize::<packets::PacketCarStatusData>(&datagram[..]).ok()
}

#[derive(Clone)]
pub(crate) struct Frame {
    pub header: Header,
    pub session_data: Option<SessionData>,
    pub lap_data: Option<LapData>,
    pub car_motion: Option<CarMotion>,
    pub car_telemetry: Option<CarTelemetry>,
    pub car_status: Option<CarStatus>,
}

impl Frame {
    pub fn new(header: &packets::PacketHeader) -> Frame {
        Frame {
            header: header.to_model(),
            session_data: None,
            lap_data: None,
            car_motion: None,
            car_telemetry: None,
            car_status: None,
        }
    }

    pub fn to_tick(&self) -> Tick {
        assert!(self.lap_data.is_some());
        assert!(self.car_motion.is_some());
        assert!(self.car_telemetry.is_some());

        Tick::new(
            self.header.clone(),
            self.session_data.clone(),
            self.lap_data.clone().unwrap(),
            self.car_motion.clone().unwrap(),
            self.car_telemetry.clone().unwrap(),
            self.car_status.clone(),
        )
    }
}
