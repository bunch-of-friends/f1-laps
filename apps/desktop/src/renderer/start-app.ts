import * as JSBridgeCore from 'f1-laps-js-bridge';
import { app, View } from 'hyperapp';
import { AppActions } from './app-actions';
import { AppState } from './app-state';
import { LapTick } from 'f1-laps-js-bridge';

const DATA_UPDATE_INTERVAL = 20;

export interface AppContext {
    liveDataBuffer: Array<LapTick>;
    lastUpdateTime: number;
}

const updatePlots = (
    context: AppContext,
    boundActions: AppActions
) => (timestamp: number) => {
    boundActions.liveData.frameUpdate(timestamp);
    const shouldUpdateData = timestamp - context.lastUpdateTime > DATA_UPDATE_INTERVAL;
    const hasNewData = context.liveDataBuffer.length > 0;
    if (shouldUpdateData && hasNewData) {
        boundActions.liveData.liveDataReceived(context.liveDataBuffer);

        context.liveDataBuffer = [];
        context.lastUpdateTime = timestamp;
    }

    requestAnimationFrame(updatePlots(context, boundActions));
}

export function startApp(
    core: typeof JSBridgeCore,
    state: AppState,
    actions: AppActions,
    view: View<AppState, AppActions>,
    container: Element | null,
    context: AppContext
) {
    core.initialise({ updateInterval: 30, storagePath: '../../_data-storage' });
    const boundActions = app(state, actions, view, container);

    core.liveData.register(data => {
        context.liveDataBuffer.push(data);
    });

    context.lastUpdateTime = 0;
    requestAnimationFrame(updatePlots(context, boundActions));

    (window as any).gs = boundActions.getState; // Debugging
}
