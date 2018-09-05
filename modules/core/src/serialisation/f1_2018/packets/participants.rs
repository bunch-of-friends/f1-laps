use serialisation::f1_2018::packets::PacketHeader;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PacketParticipantsInfo {
    pub m_header: PacketHeader,

    pub m_numCars: u8, // Number of cars in the data
    pub m_participants: [ParticipantInfoItem; 20],
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ParticipantInfoItem {
    pub m_aiControlled: u8, // Whether the vehicle is AI (1) or Human (0) controlled
    pub m_driverId: u8,     // Driver id - see appendix
    pub m_teamId: u8,       // Team id - see appendix
    pub m_raceNumber: u8,   // Race number of the car
    pub m_nationality: u8,  // Nationality of the driver

    // thi is actually array of 48, but that's too bit for serialisation, see https://github.com/serde-rs/serde/issues/631
    pub m_name: [[u8; 16]; 3], // Name of participant in UTF-8 format – null terminated
                               // Will be truncated with … (U+2026) if too long
}
