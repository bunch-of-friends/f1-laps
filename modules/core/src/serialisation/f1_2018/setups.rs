use serialisation::f1_2018::header::PacketHeader;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PacketCarSetupData {
    pub m_header: PacketHeader, // Header

    pub m_carSetups: [CarSetupData; 20],
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CarSetupData {
    pub m_frontWing: u8,             // Front wing aero
    pub m_rearWing: u8,              // Rear wing aero
    pub m_onThrottle: u8,            // Differential adjustment on throttle (percentage)
    pub m_offThrottle: u8,           // Differential adjustment off throttle (percentage)
    pub m_frontCamber: f32,          // Front camber angle (suspension geometry)
    pub m_rearCamber: f32,           // Rear camber angle (suspension geometry)
    pub m_frontToe: f32,             // Front toe angle (suspension geometry)
    pub m_rearToe: f32,              // Rear toe angle (suspension geometry)
    pub m_frontSuspension: u8,       // Front suspension
    pub m_rearSuspension: u8,        // Rear suspension
    pub m_frontAntiRollBar: u8,      // Front anti-roll bar
    pub m_rearAntiRollBar: u8,       // Front anti-roll bar
    pub m_frontSuspensionHeight: u8, // Front ride height
    pub m_rearSuspensionHeight: u8,  // Rear ride height
    pub m_brakePressure: u8,         // Brake pressure (percentage)
    pub m_brakeBias: u8,             // Brake bias (percentage)
    pub m_frontTyrePressure: f32,    // Front tyre pressure (PSI)
    pub m_rearTyrePressure: f32,     // Rear tyre pressure (PSI)
    pub m_ballast: u8,               // Ballast
    pub m_fuelLoad: f32,             // Fuel load
}