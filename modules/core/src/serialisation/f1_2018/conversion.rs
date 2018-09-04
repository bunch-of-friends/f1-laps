use pipeline::input::*;
use serialisation::f1_2018::packets::*;
use std::str;

impl PacketHeader {
    pub fn to_model(&self) -> Header {
        Header {
            session_uid: self.m_sessionUID,
            session_time: self.m_sessionTime,
            player_index: self.m_playerCarIndex,
            frame_id: self.m_frameIdentifier,
        }
    }
}

impl PacketMotionData {
    pub fn to_model(&self, header: &PacketHeader) -> ParticipantsData<CarMotion> {
        to_participants_data(header, &self.m_carMotionData, |x| CarMotion {
            x: x.m_worldPositionX,
            y: x.m_worldPositionY,
            z: x.m_worldPositionZ,
            g_force_lateral: x.m_gForceLateral,
            g_force_longitudinal: x.m_gForceLongitudinal,
            g_force_vertical: x.m_gForceVertical,
        })
    }
}

impl PacketCarStatusData {
    pub fn to_model(&self, header: &PacketHeader) -> ParticipantsData<CarStatus> {
        to_participants_data(header, &self.m_carStatusData, |x| CarStatus {
            traction_control: x.m_tractionControl,
            antilock_brakes: x.m_antiLockBrakes,
            fuel_mix: x.m_fuelMix,
            front_brake_bias: x.m_frontBrakeBias,
            pit_limiter_status: x.m_pitLimiterStatus,
            fuel_in_tank: x.m_fuelInTank,
            fuel_capacity: x.m_fuelCapacity,
            max_rpm: x.m_maxRPM,
            max_gears: x.m_maxGears,
            is_drs_allowed: x.m_drsAllowed == 1,
            tyres_wear: x.m_tyresWear,
            tyre_compound: x.m_tyreCompound,
            tyres_damage: x.m_tyresDamage,
            front_left_wing_damage: x.m_frontLeftWingDamage,
            front_right_wing_damage: x.m_frontRightWingDamage,
            rear_wing_damage: x.m_rearWingDamage,
            engine_damage: x.m_engineDamage,
            gearbox_damage: x.m_gearBoxDamage,
            exhaust_damage: x.m_exhaustDamage,
            flags: x.m_vehicleFiaFlags,
            ers_stored: x.m_ersStoreEnergy,
            ers_mode: x.m_ersDeployMode,
            ers_harvested_mghu: x.m_ersHarvestedThisLapMGUK,
            ers_harvested_mghh: x.m_ersHarvestedThisLapMGUH,
            ers_deployed: x.m_ersDeployedThisLap,
        })
    }
}

impl PacketCarTelemetryData {
    pub fn to_model(&self, header: &PacketHeader) -> ParticipantsData<CarTelemetry> {
        to_participants_data(header, &self.m_carTelemetryData, |x| CarTelemetry {
            speed: x.m_speed,
            throttle: x.m_throttle,
            steer: x.m_steer,
            brake: x.m_brake,
            gear: x.m_gear,
            clutch: x.m_clutch,
            rev_lights_percent: x.m_revLightsPercent,
            engine_rpm: x.m_engineRPM,
            is_drs_open: x.m_drs == 1,
            brakes_temperature: x.m_brakesTemperature,
            tyres_surface_temperature: x.m_tyresSurfaceTemperature,
            tyres_inner_temperature: x.m_tyresInnerTemperature,
            engine_temperature: x.m_engineTemperature,
            tyres_pressure: x.m_tyresPressure,
        })
    }
}

impl PacketLapData {
    pub fn to_model(&self, header: &PacketHeader) -> ParticipantsData<LapData> {
        to_participants_data(header, &self.m_lapData, |x| LapData {
            car_position: x.m_carPosition,
            last_lap_time: x.m_lastLapTime,
            sector1_time: x.m_sector1Time,
            sector2_time: x.m_sector2Time,
            current_sector_number: x.m_sector + 1,
            current_lap_number: x.m_currentLapNum,
            current_lap_time: x.m_currentLapTime,
            current_lap_distance: x.m_lapDistance,
            pit_status: x.m_pitStatus,
            is_lap_valid: x.m_currentLapInvalid != 0,
            penalties: x.m_penalties,
            driver_status: x.m_driverStatus,
            result_status: x.m_resultStatus,
        })
    }
}

impl PacketSessionData {
    pub fn to_model(&self) -> SessionData {
        SessionData {
            weather: self.m_weather,
            era: self.m_era,
            session_type: self.m_sessionType,
            track_id: self.m_trackId,
            track_temperature: self.m_trackTemperature,
            air_temperature: self.m_airTemperature,
            race_laps: self.m_totalLaps,
            track_length: self.m_trackLength,
            session_time_left: self.m_sessionTimeLeft,
            session_duration: self.m_sessionDuration,
            is_game_paused: self.m_gamePaused == 1,
            is_spectating: self.m_isSpectating == 1,
            is_online_game: self.m_networkGame == 1,
            safety_car_status: self.m_safetyCarStatus,
        }
    }
}

impl PacketParticipantsInfo {
    pub fn to_model(&self) -> ParticipantsData<ParticipantInfo> {
        to_participants_data(&self.m_header, &self.m_participants, |x| {
            let name_buffer: Vec<u8> = x
                .m_name
                .iter()
                .flat_map(|a| a.iter())
                .map(|x| x.clone())
                .collect();
            let name = match str::from_utf8(&name_buffer) {
                Ok(v) => v,
                Err(e) => {
                    println!("Invalid UTF-8 sequence: {}", e);
                    ""
                }
            };

            ParticipantInfo {
                is_ai: x.m_aiControlled == 1,
                driver_id: x.m_driverId,
                team_id: x.m_teamId,
                race_number: x.m_raceNumber,
                nationality_id: x.m_nationality,
                name: name.to_string(),
            }
        })
    }
}

fn to_participants_data<T, S>(
    header: &PacketHeader,
    source: &[S; 20],
    f: impl Fn(&S) -> T,
) -> ParticipantsData<T>
where
    T: Clone,
{
    let player = f(&source[header.m_playerCarIndex as usize]);
    let others = source
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != header.m_playerCarIndex as usize)
        .map(|(_, x)| f(x))
        .collect();

    ParticipantsData {
        player: player,
        others: others,
    }
}
