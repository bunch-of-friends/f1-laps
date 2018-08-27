#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PacketMotionData {
    pub m_carMotionData: [CarMotionData; 20], // Data for all cars on track

    // Extra player car ONLY data
    pub m_suspensionPosition: [f32; 4], // Note: All wheel arrays have the following order:
    pub m_suspensionVelocity: [f32; 4], // RL, RR, FL, FR
    pub m_suspensionAcceleration: [f32; 4], // RL, RR, FL, FR
    pub m_wheelSpeed: [f32; 4],         // Speed of each wheel
    pub m_wheelSlip: [f32; 4],          // Slip ratio for each wheel
    pub m_localVelocityX: f32,          // Velocity in local space
    pub m_localVelocityY: f32,          // Velocity in local space
    pub m_localVelocityZ: f32,          // Velocity in local space
    pub m_angularVelocityX: f32,        // Angular velocity x-component
    pub m_angularVelocityY: f32,        // Angular velocity y-component
    pub m_angularVelocityZ: f32,        // Angular velocity z-component
    pub m_angularAccelerationX: f32,    // Angular velocity x-component
    pub m_angularAccelerationY: f32,    // Angular velocity y-component
    pub m_angularAccelerationZ: f32,    // Angular velocity z-component
    pub m_frontWheelsAngle: f32,        // Current front wheels angle in radians
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CarMotionData {
    pub m_worldPositionX: f32,     // World space X position
    pub m_worldPositionY: f32,     // World space Y position
    pub m_worldPositionZ: f32,     // World space Z position
    pub m_worldVelocityX: f32,     // Velocity in world space X
    pub m_worldVelocityY: f32,     // Velocity in world space Y
    pub m_worldVelocityZ: f32,     // Velocity in world space Z
    pub m_worldForwardDirX: i16,   // World space forward X direction (normalised)
    pub m_worldForwardDirY: i16,   // World space forward Y direction (normalised)
    pub m_worldForwardDirZ: i16,   // World space forward Z direction (normalised)
    pub m_worldRightDirX: i16,     // World space right X direction (normalised)
    pub m_worldRightDirY: i16,     // World space right Y direction (normalised)
    pub m_worldRightDirZ: i16,     // World space right Z direction (normalised)
    pub m_gForceLateral: f32,      // Lateral G-Force component
    pub m_gForceLongitudinal: f32, // Longitudinal G-Force component
    pub m_gForceVertical: f32,     // Vertical G-Force component
    pub m_yaw: f32,                // Yaw angle in radians
    pub m_pitch: f32,              // Pitch angle in radians
    pub m_roll: f32,               // Roll angle in radians
}
