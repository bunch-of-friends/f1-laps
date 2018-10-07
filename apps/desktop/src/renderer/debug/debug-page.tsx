import { h } from 'hyperapp';
import { ObjectView } from './object-view';

import './debug.css';
import { AppState } from '../app/app-state';

export const DebugPage = () => (state: AppState) => (
    <div class="debug-page">
        <ObjectView title="Session Identifier" data={state.sessionIndenfier} />
        <ObjectView title="Session Data" data={state.sessionData} />
        <ObjectView title="Lap Fininished" data={state.lapFinished} />
        <ObjectView title="Sector Finished" data={state.sectorFinished} />
        <ObjectView title="Car Status" data={state.carStatus} />
        <ObjectView title="Car Setup" data={state.carSetup} />
        <ObjectView
            title="Live Telemetry"
            data={
                state.liveTelemetry.ticks[state.liveTelemetry.ticks.length - 1]
            }
        />
    </div>
);
