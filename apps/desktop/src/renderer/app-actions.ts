import { AppState, LiveData, ActivePlots, FPSState } from './app-state';
import { LiveTelemetryTick } from 'f1-laps-js-bridge';

const TIME_RANGE = 100;
const FPS_UPDATE_INTERVAL = 0.5;

function latestLapTick(lapTicks: Array<LiveTelemetryTick>) {
    return lapTicks[lapTicks.length - 1];
}

function filterInvisible(lapTicks: Array<LiveTelemetryTick>) {
    const currentTime =
        lapTicks.length > 0
            ? latestLapTick(lapTicks).lapData.player.current_lap_time
            : 0;
    const firstVisible = lapTicks.findIndex(
        a => a.lapData.player.current_lap_time > currentTime - TIME_RANGE
    );

    return lapTicks.slice(firstVisible);
}

function updateFPS(timeSeconds: number, fps: FPSState) {
    if (!fps.lastUpdateTime) {
        return {
            currentFPS: 0,
            lastUpdateTime: timeSeconds,
            framesSinceLastUpdate: 0,
        };
    }

    const timeSinceLastUpdate = timeSeconds - fps.lastUpdateTime;
    if (timeSinceLastUpdate > FPS_UPDATE_INTERVAL) {
        return {
            currentFPS: (fps.framesSinceLastUpdate + 1) / timeSinceLastUpdate,
            lastUpdateTime: timeSeconds,
            framesSinceLastUpdate: 0,
        };
    }

    return {
        lastUpdateTime: fps.lastUpdateTime,
        currentFPS: fps.currentFPS,
        framesSinceLastUpdate: fps.framesSinceLastUpdate + 1,
    };
}

export const appActions = {
    liveData: {
        liveDataReceived: (newLapTicks: Array<LiveTelemetryTick>) => ({
            lapTicks,
            currentLap,
            wallClockStartTime,
            wallClockTime,
        }: LiveData) => {
            let allLapTicks = lapTicks.concat(newLapTicks);
            const latestLap = latestLapTick(allLapTicks).lapData.player
                .current_lap_number;
            const lapChanged = latestLap !== currentLap;
            if (lapChanged) {
                allLapTicks = newLapTicks.filter(
                    lapTick =>
                        lapTick.lapData.player.current_lap_number === latestLap
                );
            }

            return {
                anyDataReceived: true,
                lapTicks: filterInvisible(allLapTicks),
                currentLap: latestLap,
                wallClockStartTime: lapChanged
                    ? wallClockTime
                    : wallClockStartTime,
            };
        },
        frameUpdate: (timestamp: number) => ({
            wallClockStartTime,
            anyDataReceived,
            fps,
        }: LiveData) => {
            const timeSeconds = timestamp / 1000;
            return anyDataReceived
                ? {
                      wallClockStartTime: wallClockStartTime
                          ? wallClockStartTime
                          : timeSeconds,
                      wallClockTime: timeSeconds,
                      fps: updateFPS(timeSeconds, fps),
                  }
                : null;
        },
    },
    activePlots: {
        plotActive: ({
            key,
            activePlot,
        }: {
            key: string;
            activePlot: Chart;
        }) => (activePlots: ActivePlots) => ({
            [key]: {
                instance: activePlot,
                displayedPoints: 0,
            },
        }),
        displayedPointsChanged: ({
            key,
            displayedPoints,
        }: {
            key: string;
            displayedPoints: number;
        }) => (activePlots: ActivePlots) => ({
            [key]: {
                instance: activePlots[key].instance,
                displayedPoints,
            },
        }),
    },
    getState: () => (state: AppState) => state,
};

export type AppActions = typeof appActions;
