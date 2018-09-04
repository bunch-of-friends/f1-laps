const core = require('../native') as Core;
const stayAwake = require('stay-awake');

import { createSubject, createObservable } from '@bunch-of-friends/observable';
import { MultiCarData, SessionIdentifier, SessionData, Lap, Sector, CarStatus, CarTelemetry, CarMotion, ParticipantInfo, Core, LapData } from './types';

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
let lapDataSubject = createSubject<MultiCarData<LapData>>();
let lapDataObservable = createObservable<MultiCarData<LapData>>(lapDataSubject);
let carStatusSubject = createSubject<MultiCarData<CarStatus>>();
let carStatusObservable = createObservable<MultiCarData<CarStatus>>(carStatusSubject);
let carTelemetrySubject = createSubject<MultiCarData<CarTelemetry>>();
let carTelemetryObservable = createObservable<MultiCarData<CarTelemetry>>(carTelemetrySubject);
let carMotionSubject = createSubject<MultiCarData<CarMotion>>();
let carMotionObservable = createObservable<MultiCarData<CarMotion>>(carMotionSubject);
let participantsInfoSubject = createSubject<MultiCarData<ParticipantInfo>>();
let participantsInfoObservable = createObservable<MultiCarData<ParticipantInfo>>(participantsInfoSubject);

let initialised = false;

export {
    sessionIdentifierObservable as newSession,
    lapFinishedObservable as lapFinished,
    sectorFinishedObservable as sectorFinished,
    sessionDataObservable as sessionData,
    lapDataObservable as lapData,
    carStatusObservable as carStatus,
    carTelemetryObservable as carTelemetry,
    carMotionObservable as carMotion,
    participantsInfoObservable as participantsInfo
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

    if (tick.lapData) {
        lapDataSubject.notifyObservers(tick.lapData);
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

    if (tick.participants) {
        participantsInfoSubject.notifyObservers(tick.participants);
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
