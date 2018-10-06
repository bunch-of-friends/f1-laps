import * as core from 'f1-laps-js-bridge';
import { startApp, AppContext } from './start-app';
import { appInitialState } from './app-state';
import { appActions } from './app-actions';
import { AppContainer } from './app-container/app-container';

const context: AppContext = {
    lastUpdateTime: -1,
    liveDataBuffer: [],
};

startApp(
    core,
    appInitialState,
    appActions,
    AppContainer,
    document.getElementById('app'),
    context
);
