use serialisation::f1_2018::packets::PacketHeader;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PacketLapData {
    pub m_header: PacketHeader,

    pub m_lapData: [LapDataItem; 20], // Lap data for all cars on track
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct LapDataItem {
    pub m_lastLapTime: f32,    // Last lap time in seconds
    pub m_currentLapTime: f32, // Current time around the lap in seconds
    pub m_bestLapTime: f32,    // Best lap time of the session in seconds
    pub m_sector1Time: f32,    // Sector 1 time in seconds
    pub m_sector2Time: f32,    // Sector 2 time in seconds
    pub m_lapDistance: f32,    // Distance vehicle is around current lap in metres – could
    // be negative if line hasn’t been crossed yet
    pub m_totalDistance: f32, // Total distance travelled in session in metres – could
    // be negative if line hasn’t been crossed yet
    pub m_safetyCarDelta: f32,   // Delta in seconds for safety car
    pub m_carPosition: u8,       // Car race position
    pub m_currentLapNum: u8,     // Current lap number
    pub m_pitStatus: u8,         // 0 = none, 1 = pitting, 2 = in pit area
    pub m_sector: u8,            // 0 = sector1, 1 = sector2, 2 = sector3
    pub m_currentLapInvalid: u8, // Current lap invalid - 0 = valid, 1 = invalid
    pub m_penalties: u8,         // Accumulated time penalties in seconds to be added
    pub m_gridPosition: u8,      // Grid position the vehicle started the race in
    pub m_driverStatus: u8,      // Status of driver - 0 = in garage, 1 = flying                           // 2 = in lap, 3 = out lap, 4 = on track
    pub m_resultStatus: u8,      // Result status - 0 = invalid, 1 = inactive, 2 = active
                                 // 3 = finished, 4 = disqualified, 5 = not classified
                                 // 6 = retired
}
