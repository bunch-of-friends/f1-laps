export interface AppState {
    liveData: {
        anyDataReceived: boolean;
        currentLap?: number;
    }
};

export const appInitialState = {
    liveData: {
        anyDataReceived: false
    }
};
