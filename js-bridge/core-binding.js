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

function initialise(config = { updateInterval: 50 }) {
    stayAwake.prevent(function () {
        getNextTick();
    });

    setInterval(function () {
        stayAwake.prevent(function () {
            getNextTick();
        });
    }, config.updateInterval);
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
    if (!core) {
        throw new Error('not initialised');
    }

    core.startListening(port, shouldStoreReplay);
}

function replayStoredData() {
    if (!core) {
        throw new Error('not initialised');
    }

    core.replayData();
}

module.exports = {
    initialise,
    startListening,
    replayStoredData,
    newSession: newSessionObservable,
    liveData: liveDataObservable,
    lapFinished: lapFinishedObservable,
    sectorFinished: sectorFinishedObservable,
}
