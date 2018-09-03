use pipeline::input::*;
use serialisation::f1_2018::packets::*;

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
    pub fn to_model(&self, header: &PacketHeader) -> CarMotion {
        let ref data = self.m_carMotionData[header.m_playerCarIndex as usize];
        CarMotion {
            x: data.m_worldPositionX,
            y: data.m_worldPositionY,
            z: data.m_worldPositionZ,
            g_force_lateral: data.m_gForceLateral,
            g_force_longitudinal: data.m_gForceLongitudinal,
            g_force_vertical: data.m_gForceVertical,
        }
    }
}

impl PacketCarStatusData {
    pub fn to_model(&self, header: &PacketHeader) -> CarStatus {
        let ref data = self.m_carStatusData[header.m_playerCarIndex as usize];
        CarStatus {
            traction_control: data.m_tractionControl,
            antilock_brakes: data.m_antiLockBrakes,
            fuel_mix: data.m_fuelMix,
            front_brake_bias: data.m_frontBrakeBias,
            pit_limiter_status: data.m_pitLimiterStatus,
            fuel_in_tank: data.m_fuelInTank,
            fuel_capacity: data.m_fuelCapacity,
            max_rpm: data.m_maxRPM,
            max_gears: data.m_maxGears,
            is_drs_allowed: data.m_drsAllowed == 1,
            tyres_wear: data.m_tyresWear,
            tyre_compound: data.m_tyreCompound,
            tyres_damage: data.m_tyresDamage,
            front_left_wing_damage: data.m_frontLeftWingDamage,
            front_right_wing_damage: data.m_frontRightWingDamage,
            rear_wing_damage: data.m_rearWingDamage,
            engine_damage: data.m_engineDamage,
            gearbox_damage: data.m_gearBoxDamage,
            exhaust_damage: data.m_exhaustDamage,
            flags: data.m_vehicleFiaFlags,
            ers_stored: data.m_ersStoreEnergy,
            ers_mode: data.m_ersDeployMode,
            ers_harvested_mghu: data.m_ersHarvestedThisLapMGUK,
            ers_harvested_mghh: data.m_ersHarvestedThisLapMGUH,
            ers_deployed: data.m_ersDeployedThisLap,
        }
    }
}

impl PacketCarTelemetryData {
    pub fn to_model(&self, header: &PacketHeader) -> CarTelemetry {
        let ref data = self.m_carTelemetryData[header.m_playerCarIndex as usize];
        CarTelemetry {
            speed: data.m_speed,
            throttle: data.m_throttle,
            steer: data.m_steer,
            brake: data.m_brake,
            gear: data.m_gear,
            clutch: data.m_clutch,
            rev_lights_percent: data.m_revLightsPercent,
            engine_rpm: data.m_engineRPM,
            is_drs_open: data.m_drs == 1,
            brakes_temperature: data.m_brakesTemperature,
            tyres_surface_temperature: data.m_tyresSurfaceTemperature,
            tyres_inner_temperature: data.m_tyresInnerTemperature,
            engine_temperature: data.m_engineTemperature,
            tyres_pressure: data.m_tyresPressure,
        }
    }
}

impl PacketLapData {
    pub fn to_model(&self, header: &PacketHeader) -> LapData {
        let ref data = self.m_lapData[header.m_playerCarIndex as usize];
        LapData {
            car_position: data.m_carPosition,
            last_lap_time: data.m_lastLapTime,
            sector1_time: data.m_sector1Time,
            sector2_time: data.m_sector2Time,
            current_sector_number: data.m_sector + 1,
            current_lap_number: data.m_currentLapNum,
            current_lap_time: data.m_currentLapTime,
            current_lap_distance: data.m_lapDistance,
            pit_status: data.m_pitStatus,
            is_lap_valid: data.m_currentLapInvalid != 0,
            penalties: data.m_penalties,
            driver_status: data.m_driverStatus,
            result_status: data.m_resultStatus,
        }
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

impl PacketParticipantsData {
    pub fn to_model(&self) -> ParticipantsInfo {
        let items: Vec<ParticipantsInfoItem> = self
            .m_participants
            .iter()
            .map(|p| {
                ParticipantsInfoItem {
                    is_ai: p.m_aiControlled == 1,
                    driver_id: p.m_driverId,
                    team_id: p.m_teamId,
                    race_number: p.m_raceNumber,
                    nationality_id: p.m_nationality,
                    name: String::new(), //TODO: create string from  p.m_name
                }
            })
            .collect();

        ParticipantsInfo {
            total_cars: self.m_numCars,
            participants: items,
        }
    }
}
