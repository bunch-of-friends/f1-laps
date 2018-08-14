import { LapTick } from "f1-laps-js-bridge";

export interface LiveData {
    anyDataReceived: boolean;
    lapTicks: Array<LapTick>;
    currentLap?: number;
    wallClockStartTime?: number;
    wallClockTime?: number;
}

export type ActivePlots = { [key: string]: Chart }

export interface AppState {
    liveData: LiveData;
    activePlots: ActivePlots;
};

export const appInitialState: AppState = {
    liveData: {
        anyDataReceived: false,
        lapTicks: []
    },
    activePlots: {}
};
