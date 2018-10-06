const core = require("f1-laps-js-bridge");

console.log("node test is running");

core.initialise({ updateInterval: 50, storagePath: "../../_data-storage" });

// core.newSession.register(data => console.log('newSession >> ', data));
// core.lapFinished.register(data => console.log('lapFinished >> ', data));
// core.sectorFinished.register(data => console.log('sectorFinished >> ', data));
// core.sessionData.register(data => console.log('sessionData >> ', data));
// core.carStatus.register(data => console.log('carStatus >> ', data));
// core.carTelemetry.register(data => console.log('carTelemetry >> ', data));
// core.carMotion.register(data => console.log('carMotion >> ', data));
// core.participantsInfo.register(data => console.log('participantsInfo >> ', data));
// core.carSetup.register(data => console.log('carSetup >> ', data));

//core.startListening(20777);
//core.replayPackets();

const laps = core.getLaps();
console.log("laps >>", laps);

const telemetry = core.getStoredTelemetry(laps[0].id);
console.log("telemetry >>", telemetry);
