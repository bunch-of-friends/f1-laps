const core = require('../native');
const stayAwake = require('stay-awake');

import { createSubject } from '@bunch-of-friends/observable';
import { createObservable, Observable } from '@bunch-of-friends/observable';

let newSessionSubject = createSubject<NewSession>();
let newSessionObservable = createObservable<NewSession>(newSessionSubject);
let liveDataSubject = createSubject<LapTick>();
let liveDataObservable = createObservable<LapTick>(liveDataSubject);
let lapFinishedSubject = createSubject<LapFinished>();
let lapFinishedObservable = createObservable<LapFinished>(lapFinishedSubject);
let sectorFinishedSubject = createSubject<SectorFinished>();
let sectorFinishedObservable = createObservable<SectorFinished>(sectorFinishedSubject);

let initialised = false;

export enum DRS {
    Off = 0,
    On
}

export enum TractionControl {
    Off = 0,
    Medium,
    High
}

export enum ABS {
    Off = 0,
    On
}

export enum PitStatus {
    None = 0,
    Pitting,
    InPitArea
}

export enum Sector {
    One = 0,
    Two,
    Three
}

export enum ModernTeam {
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

export enum ClassicTeam {
    Williams1992 = 0,
    McLaren1988,
    McLaren2008,
    Ferrari2004,
    Ferrari1995,
    Ferrari2007,
    McLaren1998,
    Williams1996,
    Renault2006,
    Ferrari2002,
    Redbull2010,
    McLaren1991
}

type Team = ClassicTeam | ModernTeam;

export enum SessionType {
    Unknown = 0,
    Practice,
    Qualifying,
    Race
}

export enum DRSAllowed {
    NotAllowed = 0,
    Allowed,
    InvalidOrUnknown
}

export enum Track {
    Melbourne = 0,
    Sepang,
    Shanghai,
    Bahrain,
    Catalunya,
    Monaco,
    Montreal,
    Silverstone,
    Hockenheim,
    Hungaroring,
    Spa,
    Monza,
    Singapore,
    Suzuka,
    AbuDhabi,
    Texas,
    Brazil,
    Austria,
    Sochi,
    Mexico,
    Baku,
    BahrainShort,
    SilverstoneShort,
    TexasShort,
    SuzukaShort
}

export enum VehicleFIAFlags {
    InvalidOrUnknown = -1,
    None,
    Green,
    Blue,
    Yellow,
    Red
}

export enum Era {
    Modern = 2017,
    Classic = 1980
}

export enum TyreCompound {
    UltraSoft = 0,
    SuperSoft,
    Soft,
    Medium,
    Hard,
    Inter,
    Wet
}

export enum FuelMix {
    Lean = 0,
    Standard,
    Rich,
    Max
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

export {
    newSessionObservable as newSession,
    liveDataObservable as liveData,
    lapFinishedObservable as lapFinished,
    sectorFinishedObservable as sectorFinished
}

export function initialise(config = { updateInterval: 50, storagePath: './storage' }) {
    core.initialise(config.storagePath);

    stayAwake.prevent(function () {
        getNextTick();
    });

    setInterval(function () {
        stayAwake.prevent(function () {
            getNextTick();
        });
    }, config.updateInterval);

    initialised = true;
}

function checkInitialised() {
    if (!initialised) {
        throw new Error('not initialised');
    }
}

function getNextTick() {
    const tick = core.getNextTick();

    if (tick.liveData) {
        liveDataSubject.notifyObservers(tick.liveData);
    }

    if (tick.sessionStarted) {
        console.log('what is the session started data', tick.sessionStarted);
        newSessionSubject.notifyObservers(tick.sessionStarted);
    }

    if (tick.sectorFinished) {
        console.log('what is the sector finished data', tick.sectorFinished);
        sectorFinishedSubject.notifyObservers(tick.sectorFinished);
    }

    if (tick.lapFinished) {
        console.log('what is the lap finished data', tick.lapfinished);
        lapFinishedSubject.notifyObservers(tick.lapFinished);
    }
}

export function startListening(port = 20777, shouldStoreReplay = false) {
    checkInitialised();

    core.startListening(port, shouldStoreReplay);
}

export function replayAllLaps() {
    checkInitialised();

    core.replayAllLaps();
}

export function getLapData(identifier: string): Array<LapTick> {
    checkInitialised();

    return core.getLapData(identifier);
}

export function getAllLapsMetadata(): Array<LapMetadata> {
    checkInitialised();

    return core.getAllLapsMetadata();
}

export function replayLap(identifier: string) {
    checkInitialised();

    core.replayLap(identifier);
}
