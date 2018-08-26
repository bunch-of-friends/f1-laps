use serialisation::f1_2018::header::PacketHeader;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PacketSessionData {
    pub m_header: PacketHeader, // Header

    pub m_weather: u8, // Weather - 0 = clear, 1 = light cloud, 2 = overcast
    // 3 = light rain, 4 = heavy rain, 5 = storm
    pub m_trackTemperature: i8, // Track temp. in degrees celsius
    pub m_airTemperature: i8,   // Air temp. in degrees celsius
    pub m_totalLaps: u8,        // Total number of laps in this race
    pub m_trackLength: u16,     // Track length in metres
    pub m_sessionType: u8,      // 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P
    // 5 = Q1, 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ
    // 10 = R, 11 = R2, 12 = Time Trial
    pub m_trackId: i8,             // -1 for unknown, 0-21 for tracks, see appendix
    pub m_era: u8,                 // Era, 0 = modern, 1 = classic
    pub m_sessionTimeLeft: u16,    // Time left in session in seconds
    pub m_sessionDuration: u16,    // Session duration in seconds
    pub m_pitSpeedLimit: u8,       // Pit speed limit in kilometres per hour
    pub m_gamePaused: u8,          // Whether the game is paused
    pub m_isSpectating: u8,        // Whether the player is spectating
    pub m_spectatorCarIndex: u8,   // Index of the car being spectated
    pub m_sliProNativeSupport: u8, // SLI Pro support, 0 = inactive, 1 = active
    pub m_numMarshalZones: u8,     // Number of marshal zones to follow
    pub m_marshalZones: [MarshalZone; 21], // List of marshal zones – max 21
    pub m_safetyCarStatus: u8,     // 0 = no safety car, 1 = full safety car
    // 2 = virtual safety car
    pub m_networkGame: u8, // 0 = offline, 1 = online
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MarshalZone {
    pub m_zoneStart: f32, // Fraction (0..1) of way through the lap the marshal zone starts
    pub m_zoneFlag: i8, // -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red
}
