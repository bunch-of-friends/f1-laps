const core = require('../native') as Core;
const stayAwake = require('stay-awake');

import { createSubject, createObservable } from '@bunch-of-friends/observable';
import { MultiCarData, SessionIdentifier, SessionData, Lap, Sector, CarStatus, LiveTelemetryTick, CarSetup, ParticipantInfo, Core, StoredTelemetry, LapHeader } from './types';

export * from './types';
export * from '@bunch-of-friends/observable';

let sessionIdentifierSubject = createSubject<SessionIdentifier>();
let sessionIdentifierObservable = createObservable<SessionIdentifier>(sessionIdentifierSubject);

let lapFinishedSubject = createSubject<Lap>();
let lapFinishedObservable = createObservable(lapFinishedSubject);

let sectorFinishedSubject = createSubject<Sector>();
let sectorFinishedObservable = createObservable(sectorFinishedSubject);

let sessionDataSubject = createSubject<SessionData>();
let sessionDataObservable = createObservable(sessionDataSubject);

let liveTelemetrySubject = createSubject<LiveTelemetryTick>();
let liveTelemetryObservable = createObservable(liveTelemetrySubject);

let carStatusSubject = createSubject<MultiCarData<CarStatus>>();
let carStatusObservable = createObservable(carStatusSubject);

let carSetupSubject = createSubject<MultiCarData<CarSetup>>();
let carSetupObservable = createObservable(carSetupSubject);

let participantsInfoSubject = createSubject<MultiCarData<ParticipantInfo>>();
let participantsInfoObservable = createObservable(participantsInfoSubject);

let initialised = false;

export {
    sessionIdentifierObservable as newSession,
    lapFinishedObservable as lapFinished,
    sectorFinishedObservable as sectorFinished,
    sessionDataObservable as sessionData,
    liveTelemetryObservable as liveTelemetry,
    carStatusObservable as carStatus,
    carSetupObservable as carSetup,
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

    if (tick.lapData && tick.carTelemetry && tick.carMotion) {
        liveTelemetrySubject.notifyObservers({
            lapData: tick.lapData,
            carTelemetry: tick.carTelemetry,
            carMotion: tick.carMotion
        });
    }

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

    if (tick.carStatus) {
        carStatusSubject.notifyObservers(tick.carStatus);
    }

    if (tick.carSetup) {
        carSetupSubject.notifyObservers(tick.carSetup);
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

export function getLaps(): Array<LapHeader> {
    checkInitialised();

    return core.getLaps();
}

export function getStoredTelemetry(lapId: String): StoredTelemetry {
    checkInitialised();

    return core.getLapTelemetry(lapId);
}

export function deleteTelemetry(lapId: String) {
    checkInitialised();

    core.deleteLap(lapId);
}
