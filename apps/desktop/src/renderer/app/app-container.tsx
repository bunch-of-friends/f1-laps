import { h } from 'hyperapp';
import { Link, Route } from 'hyperapp-hash-router';

import { AppState } from './app-state';
import { TelemetryPage } from '../telemetry/telemetry-page';
import { DebugPage } from '../debug/debug-page';
import { AppActions } from './app-actions';
import { LapsPage } from '../laps/laps-page';

import './app-container.css';

export const AppContainer = (state: AppState, actions: AppActions) => {
    return (
        <div id="app-container">
            <nav>
                <Link to="/">Telemetry</Link> | <Link to="/laps">Laps</Link> |{' '}
                <Link to="/debug">Debug</Link>
            </nav>
            <div id="app-buttons">
                {getListeningButton(state, actions)}
                <button
                    disabled={state.isListening}
                    onclick={() => actions.replayPackets()}
                >
                    Replay packets
                </button>
            </div>
            <div class="page">
                <Route path="/" render={TelemetryPage} />
                <Route path="/laps" render={LapsPage} />
                <Route path="/debug" render={DebugPage} />
            </div>
        </div>
    );
};

function getListeningButton(state: AppState, actions: AppActions) {
    if (!state.isListening) {
        return (
            <button onclick={() => actions.startListening()}>
                Start Listening
            </button>
        );
    } else {
        return (
            <button onclick={() => alert('not implemented')}>
                Stop Listening
            </button>
        );
    }
}
