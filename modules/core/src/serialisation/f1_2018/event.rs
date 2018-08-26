use serialisation::f1_2018::header::PacketHeader;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PacketEventData {
    pub m_header: PacketHeader, // Header

    pub m_eventStringCode: [u8; 4], // Event string code - SSTA or SEND (session start/end)
}
