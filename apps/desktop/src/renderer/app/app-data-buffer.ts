import * as core from 'f1-laps-js-bridge';

export class AppDataBuffer {
    public lastCollectionTime: number;
    public liveTelemetry: Array<core.LiveTelemetryTick>;
    public lapFinished?: core.Lap;
    public sectorFinished?: core.Sector;
    public sessionIndenfier?: core.SessionIdentifier;
    public sessionData?: core.SessionData;
    public carStatus?: core.MultiCarData<core.CarStatus>;
    public carSetup?: core.MultiCarData<core.CarSetup>;
    public participantsInfo?: core.MultiCarData<core.ParticipantInfo>;

    constructor() {
        this.lastCollectionTime = 0;
        this.liveTelemetry = [];
    }

    public flush() {
        this.liveTelemetry = [];
        this.lapFinished = undefined;
        this.sectorFinished = undefined;
        this.sessionIndenfier = undefined;
        this.sessionData = undefined;
        this.carStatus = undefined;
        this.carSetup = undefined;
        this.participantsInfo = undefined;
    }
}

export const appDataBuffer = new AppDataBuffer();
