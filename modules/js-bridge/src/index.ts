const core = require('../native');
const stayAwake = require('stay-awake');

import { createSubject, createObservable } from '@bunch-of-friends/observable';
import { NewSession, LapTick, LapFinished, SectorFinished, LapMetadata } from './types';

export * from './types';
export * from '@bunch-of-friends/observable';

let newSessionSubject = createSubject<NewSession>();
let newSessionObservable = createObservable<NewSession>(newSessionSubject);
let liveDataSubject = createSubject<LapTick>();
let liveDataObservable = createObservable<LapTick>(liveDataSubject);
let lapFinishedSubject = createSubject<LapFinished>();
let lapFinishedObservable = createObservable<LapFinished>(lapFinishedSubject);
let sectorFinishedSubject = createSubject<SectorFinished>();
let sectorFinishedObservable = createObservable<SectorFinished>(sectorFinishedSubject);

let initialised = false;

export {
    newSessionObservable as newSession,
    liveDataObservable as liveData,
    lapFinishedObservable as lapFinished,
    sectorFinishedObservable as sectorFinished
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

    if (tick.liveData) {
        liveDataSubject.notifyObservers(tick.liveData);
    }

    if (tick.sessionStarted) {
        newSessionSubject.notifyObservers(tick.sessionStarted);
    }

    if (tick.sectorFinished) {
        sectorFinishedSubject.notifyObservers(tick.sectorFinished);
    }

    if (tick.lapFinished) {
        lapFinishedSubject.notifyObservers(tick.lapFinished);
    }
}

export function startListening(port = 20777) {
    checkInitialised();

    core.startListening(port);
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
