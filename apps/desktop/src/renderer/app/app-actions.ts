import { location } from 'hyperapp-hash-router';
import * as core from 'f1-laps-js-bridge';

import { AppState, LiveTelemetry, ActivePlots, FPSState } from './app-state';
import { AppDataBuffer } from './app-data-buffer';

const TIME_RANGE = 100;
const FPS_UPDATE_INTERVAL = 0.5;

function latestLapTick(lapTicks: Array<core.LiveTelemetryTick>) {
    return lapTicks[lapTicks.length - 1];
}

function filterInvisible(lapTicks: Array<core.LiveTelemetryTick>) {
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

export const actions = {
    startListening: () => (state: AppState) => {
        if (state.isListening) {
            return state;
        }

        core.startListening();

        return {
            isListening: true,
        };
    },
    replayPackets: core.replayPackets,
    getLaps: core.getLaps,
    getTelemetry: core.getStoredTelemetry,
    deleteTelemetry: core.deleteTelemetry,
    liveTelemetry: {
        liveTelemetryReceived: (newTicks: Array<core.LiveTelemetryTick>) => ({
            ticks,
            currentLap,
            wallClockStartTime,
            wallClockTime,
        }: LiveTelemetry) => {
            let allLapTicks = ticks.concat(newTicks);
            const latestLap = latestLapTick(allLapTicks).lapData.player
                .current_lap_number;
            const lapChanged = latestLap !== currentLap;
            if (lapChanged) {
                allLapTicks = newTicks.filter(
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
        }: LiveTelemetry) => {
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
    location: location.actions,
    onAppBufferFlushed: (buffer: AppDataBuffer) => (state: AppState) => {
        // TODO: merge with live telemetry
        return {
            lapFinished: buffer.lapFinished || state.lapFinished,
            sectorFinished: buffer.sectorFinished || state.sectorFinished,
            sessionIndenfier: buffer.sessionIndenfier || state.sessionIndenfier,
            sessionData: buffer.sessionData || state.sessionData,
            carSetup: buffer.carSetup || state.carSetup,
            carStatus: buffer.carStatus || state.carStatus,
            participantsInfo: buffer.participantsInfo || state.participantsInfo,
        };
    },
};

export type AppActions = typeof actions;
