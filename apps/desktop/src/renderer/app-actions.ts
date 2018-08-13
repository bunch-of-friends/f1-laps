import { AppState, LiveData, ActivePlots } from "./app-state";
import { LapTick } from "f1-laps-js-bridge";
const TIME_RANGE = 100;

function filterInvisible(lapTicks: Array<LapTick>) {
    const currentTime = lapTicks.length > 0 ? lapTicks[lapTicks.length - 1].currentLapTime : 0;
    const firstVisible = lapTicks.findIndex(
        a => a.currentLapTime > currentTime - TIME_RANGE
    );

    return lapTicks.slice(firstVisible);
}

function updateLapPlot(lapTicks: Array<LapTick>, newPoints: Array<LapTick>) {
    return filterInvisible(lapTicks.concat(newPoints));
}

export const appActions = {
    liveData: {
        currentLapChanged: (currentLap: number) => ({ currentLap }),
        liveDataReceived: (
            newLapTicks: Array<LapTick>
        ) => (
            { lapTicks }: LiveData
        ) => {
            return {
                anyDataReceived: true,
                lapTicks: updateLapPlot(lapTicks, newLapTicks)
            };
        }
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
