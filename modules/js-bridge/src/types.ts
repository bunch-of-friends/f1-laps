export interface Core {
    initialise(storageFolderPath: string): void;
    startListening(port: number, shouldStorePackets: boolean): void;
    replayPackets(shouldSimulateTime: boolean): void;
    getNextTick(): Tick;
    getLaps(): Array<LapHeader>;
    getLapTelemetry(lapId: string): StoredTelemetry;
    deleteLap(lapId: string): void;
}

export interface Tick {
    logs?: Array<LogItem>;
    lapData?: MultiCarData<LapData>;
    carTelemetry?: MultiCarData<CarTelemetry>;
    carMotion?: MultiCarData<CarMotion>;
    sessionIdentifier?: SessionIdentifier;
    finishedLap?: Lap;
    finishedSector?: Sector;
    sessionData?: SessionData;
    carStatus?: MultiCarData<CarStatus>;
    carSetup?: MultiCarData<CarSetup>;
    participants?: MultiCarData<ParticipantInfo>;
}

export interface LogItem {
    event: LogEvent;
    message: string;
}

export enum LogEvent {
    Debug = 'Debug',
    UserInfo = 'UserInfo',
    Error = 'Error',
}

export interface LiveTelemetryTick {
    lapData: MultiCarData<LapData>;
    carTelemetry: MultiCarData<CarTelemetry>;
    carMotion: MultiCarData<CarMotion>;
}

export interface MultiCarData<T> {
    player: T;
    others?: Array<T>;
}

export interface SessionIdentifier {
    track_id: Track;
    session_type: SessionType;
    era: Era;
    session_uid: number;
    session_time: number;
}

export interface Lap {
    lap_number: number;
    lap_time: number;
    sector_times: [number, number, number];
    is_complete: boolean;
}

export interface Sector {
    sector_number: number;
    sector_time: number;
    is_complete: boolean;
}

export interface SessionData {
    weather: Weather;
    era: Era;
    session_type: SessionType;
    track_id: Track;
    track_temperature: number;
    air_temperature: number;
    race_laps: number;
    track_length: number;
    session_time_left: number;
    session_duration: number;
    is_game_paused: boolean;
    is_spectating: boolean;
    is_online_game: boolean;
    safety_car_status: SafetyCarStatus;
}

export interface LapData {
    car_position: number;
    last_lap_time: number;
    sector1_time: number;
    sector2_time: number;
    current_sector_number: number;
    current_lap_number: number;
    current_lap_time: number;
    current_lap_distance: number;
    pit_status: number;
    is_lap_valid: boolean;
    penalties: number;
    driver_status: DriverStatus;
    result_status: ResultStatus;
}

export interface CarStatus {
    traction_control: TrackionControl;
    antilock_brakes: AntilockBrakes;
    fuel_mix: FuelMix;
    front_brake_bias: number;
    pit_limiter_status: PitLimiterStatus;
    fuel_in_tank: number;
    fuel_capacity: number;
    max_rpm: number;
    max_gears: number;
    is_drs_allowed: boolean;
    tyres_wear: [number, number, number, number];
    tyre_compound: TyreCompound;
    tyres_damage: [number, number, number, number];
    front_left_wing_damage: number;
    front_right_wing_damage: number;
    rear_wing_damage: number;
    engine_damage: number;
    gearbox_damage: number;
    exhaust_damage: number;
    flags: Flag;
    ers_stored: number;
    ers_mode: ErsMode;
    ers_harvested_mghu: number;
    ers_harvested_mghh: number;
    ers_deployed: number;
}

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
    brakes_temperature: [number, number, number, number];
    tyres_surface_temperature: [number, number, number, number];
    tyres_inner_temperature: [number, number, number, number];
    engine_temperature: number;
    tyres_pressure: [number, number, number, number];
}

export interface CarMotion {
    x: number;
    y: number;
    z: number;
    g_force_lateral: number;
    g_force_longitudinal: number;
    g_force_vertical: number;
}

export interface CarSetup {
    front_wing: number;
    rear_wing: number;
    on_throttle: number;
    off_throttle: number;
    front_camber: number;
    rear_camber: number;
    front_toe: number;
    rear_toe: number;
    front_suspension: number;
    rear_suspension: number;
    front_anti_roll_bar: number;
    rear_anti_roll_bar: number;
    front_suspension_height: number;
    rear_suspension_height: number;
    brake_pressure: number;
    brake_bias: number;
    front_tyre_pressure: number;
    rear_tyre_pressure: number;
    ballast: number;
    fuel_load: number;
}

export interface ParticipantInfo {
    is_ai: boolean;
    driver_id: number;
    team_id: number;
    race_number: number;
    nationality_id: number;
    name: string;
}

export enum Weather {
    Clear = 0,
    LightClourd = 1,
    Overcast = 2,
    LightRain = 3,
    HeavyRain = 4,
    Storm = 5,
}

export enum SessionType {
    Unknown = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
    ShortP = 4,
    Q1 = 5,
    Q2 = 6,
    Q3 = 7,
    ShortQ = 8,
    OneShotQ = 9,
    Race = 10,
    Race2 = 11,
    TT = 12,
}

export enum TrackionControl {
    Low = 0,
    Medium = 1,
    High = 2,
}

export enum AntilockBrakes {
    Off = 0,
    On = 1,
}

export enum FuelMix {
    Lean = 0,
    Standard = 1,
    Rich = 2,
    Max = 3,
}

export enum PitLimiterStatus {
    Off = 0,
    On = 1,
}

export enum TyreCompound {
    HyperSoft = 0,
    UltraSoft = 1,
    SuperSoft = 2,
    Soft = 3,
    Medium = 4,
    Hard = 5,
    SuperHard = 6,
    Inter = 7,
    Wet = 8,
}

export enum Flag {
    Unknown = -1,
    None = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
    Red = 4,
}

export enum ErsMode {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Overtake = 4,
    Hotlap = 5,
}

export enum Era {
    Modern = 0,
    Classic = 1,
}

export enum SafetyCarStatus {
    NoSC = 0,
    Full = 1,
    VSC = 2,
}

export enum DriverStatus {
    InGarage = 0,
    FlyingLap = 1,
    InLap = 2,
    OutLap = 3,
    OnTrack = 4,
}

export enum ResultStatus {
    Invalid = 0,
    Inactive = 1,
    Active = 2,
    Finished = 3,
    Disqualified = 4,
    NotClasified = 5,
    Retired = 6,
}

export enum Track {
    Unknown = -1,
    Melbourne = 0,
    PaulRircard = 1,
    Shanghai = 2,
    Sakhir = 3,
    Catalunya = 4,
    Monaco = 5,
    Montreal = 6,
    Silverstone = 7,
    Hockenheim = 8,
    Hungaroring = 9,
    Spa = 10,
    Monza = 11,
    Singapore = 12,
    Suzuka = 13,
    AbuDhabi = 14,
    Texas = 15,
    Brazil = 16,
    Austria = 17,
    Sochi = 18,
    Mexico = 19,
    Baku = 20,
    SakhirShort = 21,
    SilverstoneShort = 22,
    TexasShort = 23,
    SuzukaShort = 24,
}

export enum Team {
    Mercedes = 0,
    Ferrari = 1,
    RedBull = 2,
    Williams = 3,
    ForceIndia = 4,
    Renault = 5,
    ToroRosso = 6,
    Haas = 7,
    McLaren = 8,
    Sauber = 9,
    McLaren1988 = 10,
    McLaren1991 = 11,
    Williams1992 = 12,
    Ferrari1995 = 13,
    Williams1996 = 14,
    McLaren1998 = 15,
    Ferrari2002 = 16,
    Ferrari2004 = 17,
    Renault2006 = 18,
    Ferrari2007 = 19,
    McLaren2008 = 20,
    RedBull2010 = 21,
    Ferrari1976 = 22,
    McLaren1976 = 34,
    Lotus1972 = 35,
    Ferrari1979 = 36,
    McLaren1982 = 37,
    Williams2003 = 38,
    Brawn2009 = 39,
    Lotus1978 = 40,
}

export interface LapHeader {
    id: string;
    recorded_date: number;
    track_id: Track;
    team_id: Team;
    era: Era;
    tyre_compound: TyreCompound;
    weather: Weather;
    session_type: SessionType;
    lap_number: number;
    lap_time: number;
    sector_times: [number, number, number];
    note: string;
}

export interface StoredTelemetry {
    id: string;
    session_data: SessionData;
    lap_data: Array<LapData>;
    car_status: Array<CarStatus>;
    car_telemetry: Array<CarTelemetry>;
    car_motion: Array<CarMotion>;
    car_setup: CarSetup;
    participants_info: ParticipantInfo;
}
