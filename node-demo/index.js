const core = require('f1-laps-js-bridge');

console.log('node test is running');

core.initialise({ updateInterval: 50 });

core.newSession.register(data => console.log('newSession >> ', data));
core.liveData.register(data => console.log('liveData >> ', data));
core.bestSessionLap.register(data => console.log('bestSessionLap >> ', data));
core.bestSessionSector.register(data => console.log('bestSessionSector >> ', data));
core.bestEverLap.register(data => console.log('bestEverLap >> ', data));

// core.startListening(20777, true);
core.replayStoredData(30);
