const core = require('../native');
const stayAwake = require('stay-awake');

import { createSubject, createObservable } from '@bunch-of-friends/observable';
import { NewSession, LapTick, LapFinished, SectorFinished, LapMetadata } from './types';

export * from './types';
export * from '@bunch-of-friends/observable';

let sessionIdentifierSubject = createSubject<any>();
let sessionIdentifierObservable = createObservable<any>(sessionIdentifierSubject);
let lapFinishedSubject = createSubject<any>();
let lapFinishedObservable = createObservable<any>(lapFinishedSubject);
let sectorFinishedSubject = createSubject<any>();
let sectorFinishedObservable = createObservable<any>(sectorFinishedSubject);
let sessionDataSubject = createSubject<any>();
let sessionDataObservable = createObservable<any>(sessionDataSubject);
let carStatusSubject = createSubject<any>();
let carStatusObservable = createObservable<any>(sessionDataSubject);
let carTelemetrySubject = createSubject<any>();
let carTelemetryObservable = createObservable<any>(sessionDataSubject);
let carMotionSubject = createSubject<any>();
let carMotionObservable = createObservable<any>(sessionDataSubject);

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
