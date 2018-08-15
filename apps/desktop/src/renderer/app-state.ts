import { LapTick } from "f1-laps-js-bridge";

export interface FPSState {
    lastUpdateTime?: number;
    framesSinceLastUpdate?: number;
    currentFPS?: number;
}

export interface LiveData {
    anyDataReceived: boolean;
    lapTicks: Array<LapTick>;
    currentLap?: number;
    wallClockStartTime?: number;
    wallClockTime?: number;
    fps: FPSState;
}

export interface ActivePlot {
    instance: Chart;
    displayedPoints: number;
}

export type ActivePlots = {
    [key: string]: ActivePlot
}

export interface AppState {
    liveData: LiveData;
    activePlots: ActivePlots;
};

export const appInitialState: AppState = {
    liveData: {
        anyDataReceived: false,
        lapTicks: [],
        fps: {},
    },
    activePlots: {}
};
