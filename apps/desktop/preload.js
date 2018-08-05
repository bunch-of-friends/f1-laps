const f1 = require('f1-laps-js-bridge');

f1.initialise({ updateInterval: 30, storagePath: '../_data-storage' });

window.f1 = {
    newSession: f1.newSession,
    liveData: f1.liveData,
    replayAllLaps: f1.replayAllLaps
};
