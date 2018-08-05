import { Observable } from '@bunch-of-friends/observable';
declare let newSessionObservable: Observable<NewSession>;
declare let liveDataObservable: Observable<LapTick>;
declare let lapFinishedObservable: Observable<LapFinished>;
declare let sectorFinishedObservable: Observable<SectorFinished>;
export declare enum DRS {
    Off = 0,
    On = 1
}
export declare enum TractionControl {
    Off = 0,
    Medium = 1,
    High = 2
}
export declare enum ABS {
    Off = 0,
    On = 1
}
export declare enum PitStatus {
    None = 0,
    Pitting = 1,
    InPitArea = 2
}
export declare enum Sector {
    One = 0,
    Two = 1,
    Three = 2
}
export declare enum ModernTeam {
    Redbull = 0,
    Ferrari = 1,
    McLaren = 2,
    Renault = 3,
    Mercedes = 4,
    Sauber = 5,
    ForceIndia = 6,
    Williams = 7,
    ToroRosso = 8,
    Haas = 11
}
export declare enum ClassicTeam {
    Williams1992 = 0,
    McLaren1988 = 1,
    McLaren2008 = 2,
    Ferrari2004 = 3,
    Ferrari1995 = 4,
    Ferrari2007 = 5,
    McLaren1998 = 6,
    Williams1996 = 7,
    Renault2006 = 8,
    Ferrari2002 = 9,
    Redbull2010 = 10,
    McLaren1991 = 11
}
export declare type Team = ClassicTeam | ModernTeam;
export declare enum SessionType {
    Unknown = 0,
    Practice = 1,
    Qualifying = 2,
    Race = 3
}
export declare enum DRSAllowed {
    NotAllowed = 0,
    Allowed = 1,
    InvalidOrUnknown = 2
}
export declare enum Track {
    Melbourne = 0,
    Sepang = 1,
    Shanghai = 2,
    Bahrain = 3,
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
    BahrainShort = 21,
    SilverstoneShort = 22,
    TexasShort = 23,
    SuzukaShort = 24
}
export declare enum VehicleFIAFlags {
    InvalidOrUnknown = -1,
    None = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
    Red = 4
}
export declare enum Era {
    Modern = 2017,
    Classic = 1980
}
export declare enum TyreCompound {
    UltraSoft = 0,
    SuperSoft = 1,
    Soft = 2,
    Medium = 3,
    Hard = 4,
    Inter = 5,
    Wet = 6
}
export declare enum FuelMix {
    Lean = 0,
    Standard = 1,
    Rich = 2,
    Max = 3
}
export interface LapMetadata {
    identifier: string;
    recordedDate: string;
    trackId: Track;
    teamId: Team;
    era: Era;
    tyreCompound: TyreCompound;
    sessionType: SessionType;
    lapNumber: number;
    lapTime: number;
    note: string;
    sectorTimes: Array<number>;
}
export interface CarTick {
    worldPosition: Array<number>;
    lastLapTime: number;
    currentLapTime: number;
    bestLapTime: number;
    sector1Time: number;
    sector2Time: number;
    lapDistance: number;
    driverId: number;
    teamId: number;
    carPosition: number;
    currentLapNum: number;
    inPits: PitStatus;
    sector: Sector;
    currentLapInvalid: boolean;
    penalties: number;
}
export interface LapTick {
    currentLap: number;
    currentLapTime: number;
    currentSector: Sector;
    currentSpeed: number;
    currentGear: number;
    currentTyreCompound: number;
    isLapValid: boolean;
    lastLapTime: number;
    currentLapSector1Time: number;
    totalSessionTime: number;
    totalSessionDistance: number;
    x: number;
    y: number;
    z: number;
    sessionTime: number;
    sessionTimeLeft: number;
    lapDistance: number;
    totalDistance: number;
    totalLaps: number;
    carPosition: number;
    inPits: number;
    pitLimiterStatus: boolean;
    pitSpeedLimit: number;
    drs: boolean;
    drsAllowed: DRSAllowed;
    vehicleFiaFlags: VehicleFIAFlags;
    throttle: number;
    steer: number;
    brake: number;
    gforceLat: number;
    gforceLon: number;
    gforceVert: number;
    engineRate: number;
    revLightsPercent: number;
    maxRpm: number;
    idleRpm: number;
    maxGears: number;
    tractionControl: TractionControl;
    antiLockBrakes: ABS;
    frontBrakeBias: number;
    fuelInTank: number;
    fuelCapacity: number;
    fuelMix: FuelMix;
    engineTemperature: number;
    brakesTemperature: Array<number>;
    tyresPressure: Array<number>;
    tyresTemperature: Array<number>;
    tyresWear: Array<number>;
    tyreCompound: number;
    tyresDamage: Array<number>;
    frontLeftWingDamage: number;
    frontRightWingDamage: number;
    rearWingDamage: number;
    engineDamage: number;
    gearBoxDamage: number;
    exhaustDamage: number;
    carsTotal: number;
    playerCarIndex: number;
    carData: Array<CarTick>;
}
export interface NewSession {
    sessionTimeStamp: number;
    era: Era;
    trackId: Track;
    teamId: Team;
    sessionType: SessionType;
}
export interface RecordsMarker {
    isBestEverPersonal: boolean;
    isBestEverCompoundPersonal: boolean;
    isBestSessionPersonal: boolean;
    isBestSessionPersonalCompound: boolean;
    isBestSessionAll: boolean;
    isBestSessionAllCompound: boolean;
}
export interface SectorFinished {
    sessionTimeStamp: number;
    sector: Sector;
    sectorTime: number;
    tyreCompound: TyreCompound;
    recordsMarker: RecordsMarker;
}
export interface LapFinished {
    sessionTimeStamp: number;
    lapNumber: number;
    lapTime: number;
    sector1Time: number;
    sector2Time: number;
    sector3Time: number;
    tyreCompound: TyreCompound;
    recordMarker: RecordsMarker;
}
export { newSessionObservable as newSession, liveDataObservable as liveData, lapFinishedObservable as lapFinished, sectorFinishedObservable as sectorFinished };
export declare function initialise(config?: {
    updateInterval: number;
    storagePath: string;
}): void;
export declare function startListening(port?: number): void;
export declare function replayAllLaps(): void;
export declare function getLapData(identifier: string): Array<LapTick>;
export declare function getAllLapsMetadata(): Array<LapMetadata>;
export declare function replayLap(identifier: string): void;
