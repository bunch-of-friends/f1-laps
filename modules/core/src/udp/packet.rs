#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct PacketHeader {
    pub m_packetFormat: u16,    // 2018
    pub m_packetVersion: u8,    // Version of this packet type, all start from 1
    pub m_packetId: u8,         // Identifier for the packet type, see below
    pub m_sessionUID: u64,      // Unique identifier for the session
    pub m_sessionTime: f32,     // Session timestamp
    pub m_frameIdentifier: u32, // Identifier for the frame the data was retrieved on
    pub m_playerCarIndex: u8,   // Index of player's car in the array
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct PacketMotionData {
    pub m_header: PacketHeader, // Header

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
struct CarMotionData {
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

struct PacketSessionData {
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

struct MarshalZone {
    pub m_zoneStart: f32, // Fraction (0..1) of way through the lap the marshal zone starts
    pub m_zoneFlag: i8, // -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red
}

struct PacketLapData {
    pub m_header: PacketHeader, // Header

    pub m_lapData: [LapData; 20], // Lap data for all cars on track
}

struct LapData {
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
    pub m_driverStatus: u8, // Status of driver - 0 = in garage, 1 = flying                           // 2 = in lap, 3 = out lap, 4 = on track
    pub m_resultStatus: u8, // Result status - 0 = invalid, 1 = inactive, 2 = active
                            // 3 = finished, 4 = disqualified, 5 = not classified
                            // 6 = retired
}

struct PacketEventData {
    pub m_header: PacketHeader, // Header

    pub m_eventStringCode: [u8; 4], // Event string code, see above
}

struct PacketParticipantsData {
    pub m_header: PacketHeader, // Header

    pub m_numCars: u8, // Number of cars in the data
    pub m_participants: [ParticipantData; 20],
}

struct ParticipantData {
    pub m_aiControlled: u8, // Whether the vehicle is AI (1) or Human (0) controlled
    pub m_driverId: u8,     // Driver id - see appendix
    pub m_teamId: u8,       // Team id - see appendix
    pub m_raceNumber: u8,   // Race number of the car
    pub m_nationality: u8,  // Nationality of the driver
    pub m_name: [char; 48], // Name of participant in UTF-8 format – null terminated
                            // Will be truncated with … (U+2026) if too long
}

struct PacketCarSetupData {
    pub m_header: PacketHeader, // Header

    pub m_carSetups: [CarSetupData; 20],
}

struct CarSetupData {
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

struct PacketCarTelemetryData {
    pub m_header: PacketHeader, // Header

    pub m_carTelemetryData: [CarTelemetryData; 20],

    pub m_buttonStatus: u32, // Bit flags specifying which buttons are being
                             // pressed currently - see appendices
}

struct CarTelemetryData {
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

struct PacketCarStatusData {
    pub m_header: PacketHeader, // Header
    pub m_carStatusData: [CarStatusData; 20],
}

struct CarStatusData {
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
