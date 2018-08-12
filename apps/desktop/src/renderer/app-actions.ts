import { AppState } from "./app-state";

export const appActions = {
    liveData: {
        liveDataReceived: () => ({ anyDataReceived: true }),
        currentLapChanged: (currentLap: number) => ({ currentLap })
    },
    getState: () => (state: AppState) => state
};

export type AppActions = typeof appActions;
