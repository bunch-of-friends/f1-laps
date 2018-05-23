const f1 = require('f1-laps-js-bridge');

f1.initialise({ updateInterval: 30 });

window.f1 = {
    newSession: f1.newSession,
    liveData: f1.liveData,
    replayStoredData: f1.replayStoredData
};
