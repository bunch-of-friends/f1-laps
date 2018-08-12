import * as JSBridgeCore from 'f1-laps-js-bridge';
import { app, View } from 'hyperapp';
import { AppActions } from './app-actions';
import { AppState } from './app-state';

export function startApp(
    core: typeof JSBridgeCore,
    state: AppState,
    actions: AppActions,
    view: View<AppState, AppActions>,
    container: Element | null
) {
    core.initialise({ updateInterval: 30, storagePath: '../../_data-storage' });
    const boundActions = app(state, actions, view, container);

    core.liveData.register(data => {
        boundActions.liveData.liveDataReceived(),
        boundActions.liveData.currentLapChanged(data.currentLap)
    });
    (window as any).gs = boundActions.getState;
}
