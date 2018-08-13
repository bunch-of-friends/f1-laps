import { LapTick } from "f1-laps-js-bridge";

export interface LiveData {
    anyDataReceived: boolean;
    currentLap?: number;
    lapTicks: Array<LapTick>;
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
