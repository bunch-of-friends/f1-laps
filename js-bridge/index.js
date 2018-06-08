var core = require('./native');

const stayAwake = require('stay-awake');

const createSubject = require('@bunch-of-friends/observable').createSubject;
const createObservable = require('@bunch-of-friends/observable').createObservable;

let newSessionSubject = createSubject();
let newSessionObservable = createObservable(newSessionSubject);
let liveDataSubject = createSubject();
let liveDataObservable = createObservable(liveDataSubject);
let lapFinishedSubject = createSubject();
let lapFinishedObservable = createObservable(lapFinishedSubject);
let sectorFinishedSubject = createSubject();
let sectorFinishedObservable = createObservable(sectorFinishedSubject);

let initialised = false;

function initialise(config = { updateInterval: 50, storagePath: './storage' }) {
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

function startListening(port = 20777, shouldStoreReplay = false) {
    checkInitialised();

    core.startListening(port, shouldStoreReplay);
}

function replayAllLaps() {
    checkInitialised();

    core.replayAllLaps();
}

function getLapData(identifier) {
    checkInitialised();

    return core.getLapData(identifier);
}

function getAllLapsMetadata() {
    checkInitialised();

    return core.getAllLapsMetadata();
}

function replayLap(identifier) {
    checkInitialised();

    return core.replayLap(identifier);
}

module.exports = {
    initialise,
    startListening,
    replayAllLaps,
    getLapData,
    getAllLapsMetadata,
    replayLap,
    newSession: newSessionObservable,
    liveData: liveDataObservable,
    lapFinished: lapFinishedObservable,
    sectorFinished: sectorFinishedObservable,
}