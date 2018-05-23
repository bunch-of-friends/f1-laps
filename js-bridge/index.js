const coreBinding = require('./core-binding');

module.exports = {
    initialise: coreBinding.initialise,
    startListening: coreBinding.startListening,
    replayStoredData: coreBinding.replayStoredData,
    newSession: coreBinding.newSession,
    liveData: coreBinding.liveData,
    bestSessionLap: coreBinding.bestSessionLap,
    bestSessionSector: coreBinding.bestSessionSector,
    bestEverLap: coreBinding.bestEverLap,
    bestEverSector: coreBinding.bestEverSector,
    rawData: coreBinding.rawData
}