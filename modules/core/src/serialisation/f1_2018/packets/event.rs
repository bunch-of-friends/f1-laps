use serialisation::f1_2018::packets::PacketHeader;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PacketEventData {
    pub m_header: PacketHeader,

    pub m_eventStringCode: [u8; 4], // Event string code - SSTA or SEND (session start/end)
}
