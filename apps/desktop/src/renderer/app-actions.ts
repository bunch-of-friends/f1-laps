import { AppState, LiveData, ActivePlots } from "./app-state";
import { LapTick } from "f1-laps-js-bridge";
const TIME_RANGE = 100;

function latestLapTick(lapTicks: Array<LapTick>) {
    return lapTicks[lapTicks.length - 1];
}

function filterInvisible(lapTicks: Array<LapTick>) {
    const currentTime = lapTicks.length > 0 ? latestLapTick(lapTicks).currentLapTime : 0;
    const firstVisible = lapTicks.findIndex(
        a => a.currentLapTime > currentTime - TIME_RANGE
    );

    return lapTicks.slice(firstVisible);
}

export const appActions = {
    liveData: {
        liveDataReceived: (
            newLapTicks: Array<LapTick>
        ) => (
            {
                lapTicks,
                currentLap,
                wallClockStartTime,
                wallClockTime
            }: LiveData
        ) => {
            const allLapTicks = lapTicks.concat(newLapTicks);
            const latestLap = latestLapTick(allLapTicks).currentLap;

            return {
                anyDataReceived: true,
                lapTicks: filterInvisible(allLapTicks),
                currentLap: latestLap,
                wallClockStartTime: latestLap === currentLap ? wallClockStartTime : wallClockTime

            };
        },
        frameUpdate: (
            timestamp: number
        ) => (
            {
                wallClockStartTime,
                anyDataReceived
            }: LiveData
        ) => (
            anyDataReceived ? {
                wallClockStartTime: wallClockStartTime ? wallClockStartTime : timestamp / 1000,
                wallClockTime: timestamp / 1000
            } : null
        )
    },
    activePlots: {
        plotActive: ({
            key,
            activePlot
        }: { key: string; activePlot: Chart }) => (
            activePlots: ActivePlots
        ) => ({
            ...activePlots,
            [key]: activePlot
        })
    },
    getState: () => (state: AppState) => state
};

export type AppActions = typeof appActions;
