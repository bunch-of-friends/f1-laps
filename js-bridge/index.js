const coreBinding = require('./core-binding');

module.exports = {
    initialise: coreBinding.initialise,
    startListening: coreBinding.startListening,
    replayAllLaps: coreBinding.replayAllLaps,
    newSession: coreBinding.newSession,
    liveData: coreBinding.liveData,
    lapFinished: coreBinding.lapFinished,
    sectorFinished: coreBinding.sectorFinished
}