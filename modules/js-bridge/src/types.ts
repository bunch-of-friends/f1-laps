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
    track_id: number;
    session_type: number;
    era: number;
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
    weather: number;
    era: number;
    session_type: number;
    track_id: number;
    track_temperature: number;
    air_temperature: number;
    race_laps: number;
    track_length: number;
    session_time_left: number;
    session_duration: number;
    is_game_paused: boolean;
    is_spectating: boolean;
    is_online_game: boolean;
    safety_car_status: number;
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
    driver_status: number;
    result_status: number;
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
    tyres_wear: [number, number, number, number];
    tyre_compound: number;
    tyres_damage: [number, number, number, number];
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

export interface LapHeader {
    id: string;
    recorded_date: number;
    track_id: number;
    team_id: number;
    era: number;
    tyre_compound: number;
    weather: number;
    session_type: number;
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
