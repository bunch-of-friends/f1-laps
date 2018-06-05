const core = require('f1-laps-js-bridge');

console.log('node test is running');

core.initialise({ updateInterval: 50 });

core.newSession.register(data => console.log('newSession >> ', data));
// core.liveData.register(data => console.log('liveData >> ', data.sessionTime));
core.lapFinished.register(data => console.log('lapFinished >> ', data));
core.sectorFinished.register(data => console.log('sectorFinished >> ', data));

// core.startListening(20777, true);
// core.replayAllLaps();

let metadata = core.getAllLapsMetadata();
console.log("metadata >>", metadata);
let lapData = core.getLapData(metadata[0].identifier);
console.log("lap data length >>", lapData.length);
core.replayLap(metadata[0].identifier);