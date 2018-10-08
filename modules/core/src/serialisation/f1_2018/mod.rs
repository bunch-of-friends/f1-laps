#![allow(non_snake_case)]

use context::{AppContext, LogEvent};
use pipeline::input::*;
use serialisation::ReceivePacket;

use bincode;

mod conversion;
pub(crate) mod packets;

static HEADER_SIZE: usize = 168;
static PACKET_MAX_SIZE: usize = 1341;

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
                        frame.car_motion = Some(motion.to_model());
                    }
                }
                1 => {
                    if let Some(session) = serialise_session(datagram) {
                        frame.session_data = Some(session.to_model());
                    }
                }
                2 => {
                    if let Some(lap_data) = serialise_lap_data(datagram) {
                        frame.lap_data = Some(lap_data.to_model());
                    }
                }
                3 => {
                    let _events = serialise_events(datagram);
                    // nothing for now
                }
                4 => {
                    if let Some(participants) = serialise_participants(datagram) {
                        frame.participants_info = Some(participants.to_model());
                    }
                }
                5 => {
                    if let Some(setups) = serialise_setups(datagram) {
                        frame.car_setup = Some(setups.to_model());
                    }
                }
                6 => {
                    if let Some(telemetry) = serialise_telemetry(datagram) {
                        frame.car_telemetry = Some(telemetry.to_model());
                    }
                }
                7 => {
                    if let Some(status) = serialise_status(datagram) {
                        frame.car_status = Some(status.to_model());
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
            current_frame.header.session_uid == header.m_sessionUID && current_frame.header.frame_id == header.m_frameIdentifier
        } else {
            false
        }
    }

    fn is_current_frame_complete(&self) -> bool {
        if let Some(ref current_frame) = self.current_frame {
            current_frame.is_complete()
        } else {
            false
        }
    }
}

impl ReceivePacket for Serialiser {
    fn new() -> Self {
        Serialiser { current_frame: None }
    }

    fn converto_to_tick(&mut self, context: &'static AppContext, datagram: &[u8], _size: usize) -> Option<Tick> {
        let ser_result = serialise_header(datagram);
        if ser_result.is_none() {
            context.log(LogEvent::Error, "failed to serialise packet");
        }

        let header = ser_result.unwrap();
        let mut result: Option<Tick> = None;

        if !self.is_current_frame(&header) {
            if self.is_current_frame_complete() {
                let previous_frame = self.current_frame.clone().unwrap();
                self.current_frame = Some(Frame::new(&header));
                result = previous_frame.to_tick();
            } else if self.current_frame.is_none() {
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

fn serialise_participants(datagram: &[u8]) -> Option<packets::PacketParticipantsInfo> {
    bincode::deserialize::<packets::PacketParticipantsInfo>(&datagram[..]).ok()
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
    pub lap_data: Option<MultiCarData<LapData>>,
    pub car_motion: Option<MultiCarData<CarMotion>>,
    pub car_telemetry: Option<MultiCarData<CarTelemetry>>,
    pub car_status: Option<MultiCarData<CarStatus>>,
    pub car_setup: Option<MultiCarData<CarSetup>>,
    pub participants_info: Option<MultiCarData<ParticipantInfo>>,
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
            car_setup: None,
            participants_info: None,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.lap_data.is_some() && self.car_motion.is_some() && self.car_telemetry.is_some()
    }

    pub fn to_tick(&self) -> Option<Tick> {
        assert!(self.lap_data.is_some() && self.car_motion.is_some() && self.car_telemetry.is_some());

        Some(Tick {
            header: self.header.clone(),
            session_data: self.session_data.clone(),
            lap_data: self.lap_data.clone().unwrap(),
            car_motion: self.car_motion.clone().unwrap(),
            car_telemetry: self.car_telemetry.clone().unwrap(),
            car_status: self.car_status.clone(),
            car_setup: self.car_setup.clone(),
            participants_info: self.participants_info.clone(),
        })
    }
}
