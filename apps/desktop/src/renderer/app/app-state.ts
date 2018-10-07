import * as core from 'f1-laps-js-bridge';
import { location } from 'hyperapp-hash-router';

export interface FPSState {
    lastUpdateTime: number;
    framesSinceLastUpdate: number;
    currentFPS: number;
}

export interface LiveTelemetry {
    anyDataReceived: boolean;
    ticks: Array<core.LiveTelemetryTick>;
    currentLap?: number;
    wallClockStartTime: number;
    wallClockTime: number;
    fps: FPSState;
}

export interface ActivePlot {
    instance: Chart;
    displayedPoints: number;
}

export type ActivePlots = {
    [key: string]: ActivePlot;
};

export interface AppState {
    referenceLap?: core.StoredTelemetry;
    storedLaps: Array<core.LapHeader>;
    sessionIndenfier?: core.SessionIdentifier;
    sessionData?: core.SessionData;
    lapFinished?: core.Lap;
    sectorFinished?: core.Sector;
    carStatus?: core.MultiCarData<core.CarStatus>;
    carSetup?: core.MultiCarData<core.CarSetup>;
    participantsInfo?: core.MultiCarData<core.ParticipantInfo>;
    liveTelemetry: LiveTelemetry;
    activePlots: ActivePlots;
    isListening: boolean;
    location: any;
}

export const state: AppState = {
    storedLaps: [],
    liveTelemetry: {
        wallClockStartTime: 0,
        wallClockTime: 0,
        anyDataReceived: false,
        ticks: [],
        fps: {
            lastUpdateTime: 0,
            framesSinceLastUpdate: 0,
            currentFPS: 0,
        },
    },
    activePlots: {},
    isListening: false,
    location: location.state,
};
