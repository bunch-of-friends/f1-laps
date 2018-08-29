#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PacketHeader {
    pub m_packetFormat: u16,    // 2018
    pub m_packetVersion: u8,    // Version of this packet type, all start from 1
    pub m_packetId: u8,         // Identifier for the packet type, see below
    pub m_sessionUID: u64,      // Unique identifier for the session
    pub m_sessionTime: f32,     // Session timestamp
    pub m_frameIdentifier: u32, // Identifier for the frame the data was retrieved on
    pub m_playerCarIndex: u8,   // Index of player's car in the array
}
