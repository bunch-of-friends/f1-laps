use serialisation::f1_2018::packets::PacketHeader;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PacketCarTelemetryData {
    pub m_header: PacketHeader,

    pub m_carTelemetryData: [CarTelemetryData; 20],

    pub m_buttonStatus: u32, // Bit flags specifying which buttons are being
                             // pressed currently - see appendices
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CarTelemetryData {
    pub m_speed: u16,                        // Speed of car in kilometres per hour
    pub m_throttle: u8,                      // Amount of throttle applied (0 to 100)
    pub m_steer: i8,      // Steering (-100 (full lock left) to 100 (full lock right))
    pub m_brake: u8,      // Amount of brake applied (0 to 100)
    pub m_clutch: u8,     // Amount of clutch applied (0 to 100)
    pub m_gear: i8,       // Gear selected (1-8, N=0, R=-1)
    pub m_engineRPM: u16, // Engine RPM
    pub m_drs: u8,        // 0 = off, 1 = on
    pub m_revLightsPercent: u8, // Rev lights indicator (percentage)
    pub m_brakesTemperature: [u16; 4], // Brakes temperature (celsius)
    pub m_tyresSurfaceTemperature: [u16; 4], // Tyres surface temperature (celsius)
    pub m_tyresInnerTemperature: [u16; 4], // Tyres inner temperature (celsius)
    pub m_engineTemperature: u16, // Engine temperature (celsius)
    pub m_tyresPressure: [f32; 4], // Tyres pressure (PSI)
}
