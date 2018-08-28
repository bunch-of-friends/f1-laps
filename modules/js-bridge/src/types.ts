// export enum DRS {
//     Off = 0,
//     On
// }

// export enum TractionControl {
//     Off = 0,
//     Medium,
//     High
// }

// export enum ABS {
//     Off = 0,
//     On
// }

// export enum PitStatus {
//     None = 0,
//     Pitting,
//     InPitArea
// }

// export enum Sector {
//     One = 0,
//     Two,
//     Three
// }

// export enum ModernTeam {
//     Redbull = 0,
//     Ferrari = 1,
//     McLaren = 2,
//     Renault = 3,
//     Mercedes = 4,
//     Sauber = 5,
//     ForceIndia = 6,
//     Williams = 7,
//     ToroRosso = 8,
//     Haas = 11
// }

// export enum ClassicTeam {
//     Williams1992 = 0,
//     McLaren1988,
//     McLaren2008,
//     Ferrari2004,
//     Ferrari1995,
//     Ferrari2007,
//     McLaren1998,
//     Williams1996,
//     Renault2006,
//     Ferrari2002,
//     Redbull2010,
//     McLaren1991
// }

// export type Team = ClassicTeam | ModernTeam;

// export enum SessionType {
//     Unknown = 0,
//     Practice,
//     Qualifying,
//     Race
// }

// export enum DRSAllowed {
//     NotAllowed = 0,
//     Allowed,
//     InvalidOrUnknown
// }

// export enum Track {
//     Melbourne = 0,
//     Sepang,
//     Shanghai,
//     Bahrain,
//     Catalunya,
//     Monaco,
//     Montreal,
//     Silverstone,
//     Hockenheim,
//     Hungaroring,
//     Spa,
//     Monza,
//     Singapore,
//     Suzuka,
//     AbuDhabi,
//     Texas,
//     Brazil,
//     Austria,
//     Sochi,
//     Mexico,
//     Baku,
//     BahrainShort,
//     SilverstoneShort,
//     TexasShort,
//     SuzukaShort
// }

// export enum VehicleFIAFlags {
//     InvalidOrUnknown = -1,
//     None,
//     Green,
//     Blue,
//     Yellow,
//     Red
// }

// export enum Era {
//     Modern = 2017,
//     Classic = 1980
// }

// export enum TyreCompound {
//     UltraSoft = 0,
//     SuperSoft,
//     Soft,
//     Medium,
//     Hard,
//     Inter,
//     Wet
// }

// export enum FuelMix {
//     Lean = 0,
//     Standard,
//     Rich,
//     Max
// }

// export interface LapMetadata {
//     identifier: string;
//     recordedDate: string;
//     trackId: Track;
//     teamId: Team;
//     era: Era;
//     tyreCompound: TyreCompound;
//     sessionType: SessionType;
//     lapNumber: number;
//     lapTime: number;
//     note: string;
//     sectorTimes: Array<number>;
// }

export interface CarTelemetry {
    speed: number;
    throttle: number;
    steer: number;
    brake: number;
    gear: number;
    clutch: number;
    rev_lights_percent: number;
    engine_rpm: number;
    is_drs_open: boolean;
    brakes_temperature: Array<number>;
    tyres_surface_temperature: Array<number>;
    tyres_inner_temperature: Array<number>;
    engine_temperature: number;
    tyres_pressure: Array<number>;
}

export interface CarMotion {
    x: number;
    y: number;
    z: number;
    g_force_lateral: number;
    g_force_longitudinal: number;
    g_force_vertical: number;
}

export interface CarStatus {
    traction_control: number;
    antilock_brakes: number;
    fuel_mix: number;
    front_brake_bias: number;
    pit_limiter_status: number;
    fuel_in_tank: number;
    fuel_capacity: number;
    max_rpm: number;
    max_gears: number;
    is_drs_allowed: boolean;
    tyres_wear: Array<number>;
    tyre_compound: number;
    tyres_damage: Array<number>;
    front_left_wing_damage: number;
    front_right_wing_damage: number;
    rear_wing_damage: number;
    engine_damage: number;
    gearbox_damage: number;
    exhaust_damage: number;
    flags: number;
    ers_stored: number;
    ers_mode: number;
    ers_harvested_mghu: number;
    ers_harvested_mghh: number;
    ers_deployed: number;
}

export interface SessionData {
    weather: number;
    era: number;
    session_type: number;
    track_id: number;
    track_temperature: number;
    air_temperature: number;
    race_laps: number;
    track_lenght: number;
    session_time_left: number;
    session_duration: number;
    is_game_paused: boolean;
    is_spectating: boolean;
    is_online_game: boolean;
    safety_car_status: number;
}

export interface SessionIdentifier {
    track_id: number;
    session_type: number;
    era: number;
    uid: number;
}

export interface SectorFinished {
    sector_number: number;
    sector_time: number;
    is_finished: boolean;
}

export interface LapFinished {
    lap_number: number;
    lap_time: number;
    sector_times: Array<number>;
    is_finished: boolean;
}
