import * as core from 'f1-laps-js-bridge';
import { app } from 'hyperapp';
import { location } from 'hyperapp-hash-router';

import { state } from './app-state';
import { appDataBuffer, AppDataBuffer } from './app-data-buffer';
import { actions, AppActions } from './app-actions';
import { AppContainer } from './app-container';

const DATA_UPDATE_INTERVAL = 20;

export function startApp(container: Element | null) {
    if (!container) {
        throw new Error('cannot start app, container not provided');
    }

    core.initialise({ updateInterval: 30, storagePath: '../../_data-storage' });

    core.logs.register(logs => {
        appDataBuffer.logs = appDataBuffer.logs.concat(logs);
    });

    core.liveTelemetry.register(data => {
        appDataBuffer.liveTelemetry.push(data);
    });

    core.lapFinished.register(data => {
        appDataBuffer.lapFinished = data;
    });

    core.sectorFinished.register(data => {
        appDataBuffer.sectorFinished = data;
    });

    core.newSession.register(data => {
        appDataBuffer.sessionIndenfier = data;
    });

    core.sessionData.register(data => {
        appDataBuffer.sessionData = data;
    });

    core.carStatus.register(data => {
        appDataBuffer.carStatus = data;
    });

    core.carSetup.register(data => {
        appDataBuffer.carSetup = data;
    });

    core.participantsInfo.register(data => {
        appDataBuffer.participantsInfo = data;
    });

    const boundActions = app(
        state,
        actions,
        AppContainer,
        container
    ) as AppActions;

    location.subscribe(boundActions.location);

    requestAnimationFrame(updateState(appDataBuffer, boundActions));

    (window as any).gs = boundActions.getState; // debugging
}

const updateState = (buffer: AppDataBuffer, a: AppActions) => (
    timestamp: number
) => {
    a.liveTelemetry.frameUpdate(timestamp);

    const shouldUpdateData =
        timestamp - buffer.lastCollectionTime > DATA_UPDATE_INTERVAL;

    if (!shouldUpdateData) {
        requestAnimationFrame(updateState(buffer, a));
        return;
    }

    if (buffer.liveTelemetry.length) {
        a.liveTelemetry.liveTelemetryReceived(buffer.liveTelemetry);
    }

    actions.onAppBufferFlushed(appDataBuffer);
    buffer.flush();

    requestAnimationFrame(updateState(buffer, a));
};
