"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var core = require('../native');
var stayAwake = require('stay-awake');
var observable_1 = require("@bunch-of-friends/observable");
var observable_2 = require("@bunch-of-friends/observable");
var newSessionSubject = observable_1.createSubject();
var newSessionObservable = observable_2.createObservable(newSessionSubject);
exports.newSession = newSessionObservable;
var liveDataSubject = observable_1.createSubject();
var liveDataObservable = observable_2.createObservable(liveDataSubject);
exports.liveData = liveDataObservable;
var lapFinishedSubject = observable_1.createSubject();
var lapFinishedObservable = observable_2.createObservable(lapFinishedSubject);
exports.lapFinished = lapFinishedObservable;
var sectorFinishedSubject = observable_1.createSubject();
var sectorFinishedObservable = observable_2.createObservable(sectorFinishedSubject);
exports.sectorFinished = sectorFinishedObservable;
var initialised = false;
var DRS;
(function (DRS) {
    DRS[DRS["Off"] = 0] = "Off";
    DRS[DRS["On"] = 1] = "On";
})(DRS = exports.DRS || (exports.DRS = {}));
var TractionControl;
(function (TractionControl) {
    TractionControl[TractionControl["Off"] = 0] = "Off";
    TractionControl[TractionControl["Medium"] = 1] = "Medium";
    TractionControl[TractionControl["High"] = 2] = "High";
})(TractionControl = exports.TractionControl || (exports.TractionControl = {}));
var ABS;
(function (ABS) {
    ABS[ABS["Off"] = 0] = "Off";
    ABS[ABS["On"] = 1] = "On";
})(ABS = exports.ABS || (exports.ABS = {}));
var PitStatus;
(function (PitStatus) {
    PitStatus[PitStatus["None"] = 0] = "None";
    PitStatus[PitStatus["Pitting"] = 1] = "Pitting";
    PitStatus[PitStatus["InPitArea"] = 2] = "InPitArea";
})(PitStatus = exports.PitStatus || (exports.PitStatus = {}));
var Sector;
(function (Sector) {
    Sector[Sector["One"] = 0] = "One";
    Sector[Sector["Two"] = 1] = "Two";
    Sector[Sector["Three"] = 2] = "Three";
})(Sector = exports.Sector || (exports.Sector = {}));
var ModernTeam;
(function (ModernTeam) {
    ModernTeam[ModernTeam["Redbull"] = 0] = "Redbull";
    ModernTeam[ModernTeam["Ferrari"] = 1] = "Ferrari";
    ModernTeam[ModernTeam["McLaren"] = 2] = "McLaren";
    ModernTeam[ModernTeam["Renault"] = 3] = "Renault";
    ModernTeam[ModernTeam["Mercedes"] = 4] = "Mercedes";
    ModernTeam[ModernTeam["Sauber"] = 5] = "Sauber";
    ModernTeam[ModernTeam["ForceIndia"] = 6] = "ForceIndia";
    ModernTeam[ModernTeam["Williams"] = 7] = "Williams";
    ModernTeam[ModernTeam["ToroRosso"] = 8] = "ToroRosso";
    ModernTeam[ModernTeam["Haas"] = 11] = "Haas";
})(ModernTeam = exports.ModernTeam || (exports.ModernTeam = {}));
var ClassicTeam;
(function (ClassicTeam) {
    ClassicTeam[ClassicTeam["Williams1992"] = 0] = "Williams1992";
    ClassicTeam[ClassicTeam["McLaren1988"] = 1] = "McLaren1988";
    ClassicTeam[ClassicTeam["McLaren2008"] = 2] = "McLaren2008";
    ClassicTeam[ClassicTeam["Ferrari2004"] = 3] = "Ferrari2004";
    ClassicTeam[ClassicTeam["Ferrari1995"] = 4] = "Ferrari1995";
    ClassicTeam[ClassicTeam["Ferrari2007"] = 5] = "Ferrari2007";
    ClassicTeam[ClassicTeam["McLaren1998"] = 6] = "McLaren1998";
    ClassicTeam[ClassicTeam["Williams1996"] = 7] = "Williams1996";
    ClassicTeam[ClassicTeam["Renault2006"] = 8] = "Renault2006";
    ClassicTeam[ClassicTeam["Ferrari2002"] = 9] = "Ferrari2002";
    ClassicTeam[ClassicTeam["Redbull2010"] = 10] = "Redbull2010";
    ClassicTeam[ClassicTeam["McLaren1991"] = 11] = "McLaren1991";
})(ClassicTeam = exports.ClassicTeam || (exports.ClassicTeam = {}));
var SessionType;
(function (SessionType) {
    SessionType[SessionType["Unknown"] = 0] = "Unknown";
    SessionType[SessionType["Practice"] = 1] = "Practice";
    SessionType[SessionType["Qualifying"] = 2] = "Qualifying";
    SessionType[SessionType["Race"] = 3] = "Race";
})(SessionType = exports.SessionType || (exports.SessionType = {}));
var DRSAllowed;
(function (DRSAllowed) {
    DRSAllowed[DRSAllowed["NotAllowed"] = 0] = "NotAllowed";
    DRSAllowed[DRSAllowed["Allowed"] = 1] = "Allowed";
    DRSAllowed[DRSAllowed["InvalidOrUnknown"] = 2] = "InvalidOrUnknown";
})(DRSAllowed = exports.DRSAllowed || (exports.DRSAllowed = {}));
var Track;
(function (Track) {
    Track[Track["Melbourne"] = 0] = "Melbourne";
    Track[Track["Sepang"] = 1] = "Sepang";
    Track[Track["Shanghai"] = 2] = "Shanghai";
    Track[Track["Bahrain"] = 3] = "Bahrain";
    Track[Track["Catalunya"] = 4] = "Catalunya";
    Track[Track["Monaco"] = 5] = "Monaco";
    Track[Track["Montreal"] = 6] = "Montreal";
    Track[Track["Silverstone"] = 7] = "Silverstone";
    Track[Track["Hockenheim"] = 8] = "Hockenheim";
    Track[Track["Hungaroring"] = 9] = "Hungaroring";
    Track[Track["Spa"] = 10] = "Spa";
    Track[Track["Monza"] = 11] = "Monza";
    Track[Track["Singapore"] = 12] = "Singapore";
    Track[Track["Suzuka"] = 13] = "Suzuka";
    Track[Track["AbuDhabi"] = 14] = "AbuDhabi";
    Track[Track["Texas"] = 15] = "Texas";
    Track[Track["Brazil"] = 16] = "Brazil";
    Track[Track["Austria"] = 17] = "Austria";
    Track[Track["Sochi"] = 18] = "Sochi";
    Track[Track["Mexico"] = 19] = "Mexico";
    Track[Track["Baku"] = 20] = "Baku";
    Track[Track["BahrainShort"] = 21] = "BahrainShort";
    Track[Track["SilverstoneShort"] = 22] = "SilverstoneShort";
    Track[Track["TexasShort"] = 23] = "TexasShort";
    Track[Track["SuzukaShort"] = 24] = "SuzukaShort";
})(Track = exports.Track || (exports.Track = {}));
var VehicleFIAFlags;
(function (VehicleFIAFlags) {
    VehicleFIAFlags[VehicleFIAFlags["InvalidOrUnknown"] = -1] = "InvalidOrUnknown";
    VehicleFIAFlags[VehicleFIAFlags["None"] = 0] = "None";
    VehicleFIAFlags[VehicleFIAFlags["Green"] = 1] = "Green";
    VehicleFIAFlags[VehicleFIAFlags["Blue"] = 2] = "Blue";
    VehicleFIAFlags[VehicleFIAFlags["Yellow"] = 3] = "Yellow";
    VehicleFIAFlags[VehicleFIAFlags["Red"] = 4] = "Red";
})(VehicleFIAFlags = exports.VehicleFIAFlags || (exports.VehicleFIAFlags = {}));
var Era;
(function (Era) {
    Era[Era["Modern"] = 2017] = "Modern";
    Era[Era["Classic"] = 1980] = "Classic";
})(Era = exports.Era || (exports.Era = {}));
var TyreCompound;
(function (TyreCompound) {
    TyreCompound[TyreCompound["UltraSoft"] = 0] = "UltraSoft";
    TyreCompound[TyreCompound["SuperSoft"] = 1] = "SuperSoft";
    TyreCompound[TyreCompound["Soft"] = 2] = "Soft";
    TyreCompound[TyreCompound["Medium"] = 3] = "Medium";
    TyreCompound[TyreCompound["Hard"] = 4] = "Hard";
    TyreCompound[TyreCompound["Inter"] = 5] = "Inter";
    TyreCompound[TyreCompound["Wet"] = 6] = "Wet";
})(TyreCompound = exports.TyreCompound || (exports.TyreCompound = {}));
var FuelMix;
(function (FuelMix) {
    FuelMix[FuelMix["Lean"] = 0] = "Lean";
    FuelMix[FuelMix["Standard"] = 1] = "Standard";
    FuelMix[FuelMix["Rich"] = 2] = "Rich";
    FuelMix[FuelMix["Max"] = 3] = "Max";
})(FuelMix = exports.FuelMix || (exports.FuelMix = {}));
function initialise(config) {
    if (config === void 0) { config = { updateInterval: 50, storagePath: './storage' }; }
    core.initialise(config.storagePath);
    stayAwake.prevent(function () {
        getNextTick();
    });
    setInterval(function () {
        stayAwake.prevent(function () {
            getNextTick();
        });
    }, config.updateInterval);
    initialised = true;
}
exports.initialise = initialise;
function checkInitialised() {
    if (!initialised) {
        throw new Error('not initialised');
    }
}
function getNextTick() {
    var tick = core.getNextTick();
    if (tick.liveData) {
        liveDataSubject.notifyObservers(tick.liveData);
    }
    if (tick.sessionStarted) {
        newSessionSubject.notifyObservers(tick.sessionStarted);
    }
    if (tick.sectorFinished) {
        sectorFinishedSubject.notifyObservers(tick.sectorFinished);
    }
    if (tick.lapFinished) {
        lapFinishedSubject.notifyObservers(tick.lapFinished);
    }
}
function startListening(port) {
    if (port === void 0) { port = 20777; }
    checkInitialised();
    core.startListening(port);
}
exports.startListening = startListening;
function replayAllLaps() {
    checkInitialised();
    core.replayAllLaps();
}
exports.replayAllLaps = replayAllLaps;
function getLapData(identifier) {
    checkInitialised();
    return core.getLapData(identifier);
}
exports.getLapData = getLapData;
function getAllLapsMetadata() {
    checkInitialised();
    return core.getAllLapsMetadata();
}
exports.getAllLapsMetadata = getAllLapsMetadata;
function replayLap(identifier) {
    checkInitialised();
    core.replayLap(identifier);
}
exports.replayLap = replayLap;
//# sourceMappingURL=index.js.map