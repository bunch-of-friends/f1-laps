const core = require('f1-laps-js-bridge');

console.log('node test is running');

core.initialise({ updateInterval: 50 });

core.newSession.register(data => console.log('newSession >> ', data));
//core.liveData.register(data => console.log('liveData >> ', data.sessionTime));
core.lapFinished.register(data => console.log('lapFinished >> ', data));
core.sectorFinished.register(data => console.log('sectorFinished >> ', data));

// core.startListening(20777, true);
core.replayAllLaps();
