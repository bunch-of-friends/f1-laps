const core = require('f1-laps-js-bridge');

console.log('node test is running');

core.initialise({ updateInterval: 50, storagePath: '../../_data-storage' });

core.newSession.register(data => console.log('newSession >> ', data));
core.lapFinished.register(data => console.log('lapFinished >> ', data));
core.sectorFinished.register(data => console.log('sectorFinished >> ', data));
core.sessionData.register(data => console.log('sessionData >> ', data));
core.carStatus.register(data => console.log('carStatus >> ', data));
core.carTelemetry.register(data => console.log('carTelemetry >> ', data));
core.carMotion.register(data => console.log('carMotion >> ', data));
core.participantsInfo.register(data => console.log('participantsInfo >> ', data));
core.carSetup.register(data => console.log('carSetup >> ', data));

// core.startListening(20777);
core.replayPackets();

// let metadata = core.getAllLapsMetadata();
// console.log("metadata >>", metadata);
// let lapData = core.getLapData(metadata[0].identifier);
// console.log("lap data length >>", lapData.length);
// core.replayLap(metadata[0].identifier);