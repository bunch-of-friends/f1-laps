var core = require('./native');

const stayAwake = require('stay-awake');

const createSubject = require('@bunch-of-friends/observable').createSubject;
const createObservable = require('@bunch-of-friends/observable').createObservable;

let newSessionSubject = createSubject();
let newSessionObservable = createObservable(newSessionSubject);
let liveDataSubject = createSubject();
let liveDataObservable = createObservable(liveDataSubject);
let bestSessionLapSubject = createSubject();
let bestSessionLapObservable = createObservable(bestSessionLapSubject);
let bestSessionSectorSubject = createSubject();
let bestSessionSectorObservable = createObservable(bestSessionSectorSubject);
let bestEverLapSubject = createSubject();
let bestEverLapObservable = createObservable(bestEverLapSubject);
let bestEverSectorSubject = createSubject();
let bestEverSectorObservable = createObservable(bestEverSectorSubject);

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

    if (tick.session) {
        newSessionSubject.notifyObservers(tick.session);
    }

    if (tick.liveData) {
        liveDataSubject.notifyObservers(tick.liveData);
    }

    if (tick.bestSessionLap) {
        bestSessionSectorLap.notifyObservers(tick.bestSessionLap);
    }

    if (tick.bestSessionSector) {
        bestSessionSectorSubject.notifyObservers(tick.bestSessionSector);
    }

    if (tick.bestEverLap) {
        bestEverLapSubject.notifyObservers(tick.bestEverLap);
    }

    if (tick.bestEverSector) {
        bestEverSectorSubject.notifyObservers(tick.bestEverSector);
    }
}

function startListening(port = 20777, shouldStoreReplay = false) {
    if (!core) {
        throw new Error('not initialised');
    }

    core.startListening(port, shouldStoreReplay);
}

function replayStoredData(frequencyHz) {
    if (!core) {
        throw new Error('not initialised');
    }

    core.replayData(frequencyHz);
}

module.exports = {
    initialise,
    startListening,
    replayStoredData,
    newSession: newSessionObservable,
    liveData: liveDataObservable,
    bestSessionLap: bestSessionLapObservable,
    bestSessionSector: bestSessionSectorObservable,
    bestEverLap: bestEverLapObservable,
    bestEverSector: bestEverSectorObservable,
}
