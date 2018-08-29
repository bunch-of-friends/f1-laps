const core = require('../native');
const stayAwake = require('stay-awake');

import { createSubject, createObservable } from '@bunch-of-friends/observable';
import { SessionIdentifier, SessionData, LapFinished, SectorFinished, CarStatus, CarTelemetry, CarMotion } from './types';

export * from './types';
export * from '@bunch-of-friends/observable';

let sessionIdentifierSubject = createSubject<SessionIdentifier>();
let sessionIdentifierObservable = createObservable<SessionIdentifier>(sessionIdentifierSubject);
let lapFinishedSubject = createSubject<LapFinished>();
let lapFinishedObservable = createObservable<LapFinished>(lapFinishedSubject);
let sectorFinishedSubject = createSubject<SectorFinished>();
let sectorFinishedObservable = createObservable<SectorFinished>(sectorFinishedSubject);
let sessionDataSubject = createSubject<SessionData>();
let sessionDataObservable = createObservable<SessionData>(sessionDataSubject);
let carStatusSubject = createSubject<CarStatus>();
let carStatusObservable = createObservable<CarStatus>(carStatusSubject);
let carTelemetrySubject = createSubject<CarTelemetry>();
let carTelemetryObservable = createObservable<CarTelemetry>(carTelemetrySubject);
let carMotionSubject = createSubject<CarMotion>();
let carMotionObservable = createObservable<CarMotion>(carMotionSubject);

let initialised = false;

export {
    sessionIdentifierObservable as newSession,
    lapFinishedObservable as lapFinished,
    sectorFinishedObservable as sectorFinished,
    sessionDataObservable as sessionData,
    carStatusObservable as carStatus,
    carTelemetryObservable as carTelemetry,
    carMotionObservable as carMotion
};

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

    if (tick.sessionIdentifier) {
        sessionIdentifierSubject.notifyObservers(tick.sessionIdentifier);
    }

    if (tick.finishedLap) {
        lapFinishedSubject.notifyObservers(tick.finishedLap);
    }

    if (tick.finishedSector) {
        sectorFinishedSubject.notifyObservers(tick.sectorFinished);
    }

    if (tick.sessionData) {
        sessionDataSubject.notifyObservers(tick.sessionData);
    }

    if (tick.carStatus) {
        carStatusSubject.notifyObservers(tick.carStatus);
    }

    if (tick.carTelemetry) {
        carTelemetrySubject.notifyObservers(tick.carTelemetry);
    }

    if (tick.carMotion) {
        carMotionSubject.notifyObservers(tick.carMotion);
    }
}

export function startListening(port = 20777) {
    checkInitialised();

    core.startListening(port);
}

// export function replayAllLaps() {
//     checkInitialised();

//     core.replayAllLaps();
// }

// export function getLapData(identifier: string): Array<LapTick> {
//     checkInitialised();

//     return core.getLapData(identifier);
// }

// export function getAllLapsMetadata(): Array<LapMetadata> {
//     checkInitialised();

//     return core.getAllLapsMetadata();
// }

// export function replayLap(identifier: string) {
//     checkInitialised();

//     core.replayLap(identifier);
// }
