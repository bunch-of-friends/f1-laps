import { Point } from "./math/linear-algebra";

export interface LiveData {
    anyDataReceived: boolean;
    currentLap?: number;
    speed: Array<Point>;
}

export type ActivePlots = { [key: string]: Chart }

export interface AppState {
    liveData: LiveData;
    activePlots: ActivePlots;
};

export const appInitialState: AppState = {
    liveData: {
        anyDataReceived: false,
        speed: []
    },
    activePlots: {}
};
