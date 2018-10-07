import { h } from 'hyperapp';
import { ObjectView } from './object-view';

import './debug.css';
import { AppState } from '../app/app-state';

export const DebugPage = () => (state: AppState) => {
    const tick =
        state.liveTelemetry.ticks[state.liveTelemetry.ticks.length - 1];
    let tickToRender = {};
    if (tick) {
        tickToRender = {
            carMotion: tick.carMotion.player,
            lapData: tick.lapData.player,
            carTelemetry: tick.carTelemetry.player,
        };
    }
    return (
        <div class="debug-page">
            <div>
                <ObjectView title="Live Telemetry" data={tickToRender} />
            </div>
            <div>
                <ObjectView
                    title="Car Status"
                    data={state.carStatus && state.carStatus.player}
                />
                <ObjectView
                    title="Car Setup"
                    data={state.carSetup && state.carSetup.player}
                />
            </div>
            <div>
                <ObjectView title="Lap Fininished" data={state.lapFinished} />
                <ObjectView
                    title="Sector Finished"
                    data={state.sectorFinished}
                />
                <ObjectView
                    title="Session Identifier"
                    data={state.sessionIndenfier}
                />
                <ObjectView title="Session Data" data={state.sessionData} />
            </div>
        </div>
    );
};
