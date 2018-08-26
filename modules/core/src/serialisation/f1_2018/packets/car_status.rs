use serialisation::f1_2018::packets::PacketHeader;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PacketCarStatusData {
    pub m_header: PacketHeader, // Header
    pub m_carStatusData: [CarStatusData; 20],
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CarStatusData {
    pub m_tractionControl: u8,  // 0 (off) - 2 (high)
    pub m_antiLockBrakes: u8,   // 0 (off) - 1 (on)
    pub m_fuelMix: u8,          // Fuel mix - 0 = lean, 1 = standard, 2 = rich, 3 = max
    pub m_frontBrakeBias: u8,   // Front brake bias (percentage)
    pub m_pitLimiterStatus: u8, // Pit limiter status - 0 = off, 1 = on
    pub m_fuelInTank: f32,      // Current fuel mass
    pub m_fuelCapacity: f32,    // Fuel capacity
    pub m_maxRPM: u16,          // Cars max RPM, point of rev limiter
    pub m_idleRPM: u16,         // Cars idle RPM
    pub m_maxGears: u8,         // Maximum number of gears
    pub m_drsAllowed: u8,       // 0 = not allowed, 1 = allowed, -1 = unknown
    pub m_tyresWear: [u8; 4],   // Tyre wear percentage
    pub m_tyreCompound: u8,     // Modern - 0 = hyper soft, 1 = ultra soft
    // 2 = super soft, 3 = soft, 4 = medium, 5 = hard
    // 6 = super hard, 7 = inter, 8 = wet
    // Classic - 0-6 = dry, 7-8 = wet
    pub m_tyresDamage: [u8; 4],     // Tyre damage (percentage)
    pub m_frontLeftWingDamage: u8,  // Front left wing damage (percentage)
    pub m_frontRightWingDamage: u8, // Front right wing damage (percentage)
    pub m_rearWingDamage: u8,       // Rear wing damage (percentage)
    pub m_engineDamage: u8,         // Engine damage (percentage)
    pub m_gearBoxDamage: u8,        // Gear box damage (percentage)
    pub m_exhaustDamage: u8,        // Exhaust damage (percentage)
    pub m_vehicleFiaFlags: i8,      // -1 = invalid/unknown, 0 = none, 1 = green
    // 2 = blue, 3 = yellow, 4 = red
    pub m_ersStoreEnergy: f32, // ERS energy store in Joules
    pub m_ersDeployMode: u8,   // ERS deployment mode, 0 = none, 1 = low, 2 = medium
    // 3 = high, 4 = overtake, 5 = hotlap
    pub m_ersHarvestedThisLapMGUK: f32, // ERS energy harvested this lap by MGU-K
    pub m_ersHarvestedThisLapMGUH: f32, // ERS energy harvested this lap by MGU-H
    pub m_ersDeployedThisLap: f32,      // ERS energy deployed this lap
}
