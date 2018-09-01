const core = require('../native') as Core;
const stayAwake = require('stay-awake');

import { createSubject, createObservable } from '@bunch-of-friends/observable';
import { SessionIdentifier, SessionData, Lap, Sector, CarStatus, Telemetry, Core } from './types';

export * from './types';
export * from '@bunch-of-friends/observable';

let sessionIdentifierSubject = createSubject<SessionIdentifier>();
let sessionIdentifierObservable = createObservable<SessionIdentifier>(sessionIdentifierSubject);
let lapFinishedSubject = createSubject<Lap>();
let lapFinishedObservable = createObservable<Lap>(lapFinishedSubject);
let sectorFinishedSubject = createSubject<Sector>();
let sectorFinishedObservable = createObservable<Sector>(sectorFinishedSubject);
let sessionDataSubject = createSubject<SessionData>();
let sessionDataObservable = createObservable<SessionData>(sessionDataSubject);
let telemetryDataSubject = createSubject<Telemetry>();
let telemetryDataObservable = createObservable<Telemetry>(telemetryDataSubject);
let carStatusSubject = createSubject<CarStatus>();
let carStatusObservable = createObservable<CarStatus>(carStatusSubject);

let initialised = false;

export {
    sessionIdentifierObservable as newSession,
    lapFinishedObservable as lapFinished,
    sectorFinishedObservable as sectorFinished,
    sessionDataObservable as sessionData,
    telemetryDataObservable as telemetry,
    carStatusObservable as carStatus
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
        sectorFinishedSubject.notifyObservers(tick.finishedSector);
    }

    if (tick.sessionData) {
        sessionDataSubject.notifyObservers(tick.sessionData);
    }

    if (tick.carTelemetry && tick.lapData && tick.carMotion) {
        telemetryDataSubject.notifyObservers({
            carTelemetry: tick.carTelemetry,
            lapData: tick.lapData,
            carMotion: tick.carMotion
        });
    }

    if (tick.carStatus) {
        carStatusSubject.notifyObservers(tick.carStatus);
    }
}

export function startListening(port = 20777, shouldStorePackets = true) {
    checkInitialised();

    core.startListening(port, shouldStorePackets);
}

export function replayPackets(shouldSimulateTime = true) {
    checkInitialised();

    core.replayPackets(shouldSimulateTime);
}
