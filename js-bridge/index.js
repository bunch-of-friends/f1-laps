const coreBinding = require('./core-binding');

module.exports = {
    initialise: coreBinding.initialise,
    startListening: coreBinding.startListening,
    replayStoredData: coreBinding.replayStoredData,
    newSession: coreBinding.newSession,
    liveData: coreBinding.liveData,
    lapFinished: coreBinding.lapFinished,
    sectorFinished: coreBinding.sectorFinished
}