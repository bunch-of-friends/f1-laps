import { AppState, LiveData, ActivePlots } from "./app-state";
import { LapTick } from "f1-laps-js-bridge";
import { Point } from "./math/linear-algebra";

const TIME_RANGE = 100;

function filterInvisible(arr: Array<Point>, currentTime: number) {
    const firstVisible = arr.findIndex(a => a.x > currentTime - TIME_RANGE);

    return arr.slice(firstVisible);
}

function updateLapPlot(arr: Array<Point>, newPoint: Point) {
    return filterInvisible(arr, newPoint.x).concat([newPoint]);
}

export const appActions = {
    liveData: {
        liveDataReceived: () => ({ anyDataReceived: true }),
        currentLapChanged: (currentLap: number) => ({ currentLap }),
        dataPointReceived: (lapTick: LapTick) => ({ speed }: LiveData) => {
            return {
                speed: updateLapPlot(speed, {
                    x: lapTick.currentLapTime,
                    y: lapTick.currentSpeed
                })
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
