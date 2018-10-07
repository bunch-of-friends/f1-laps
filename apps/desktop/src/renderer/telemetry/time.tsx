import { h } from 'hyperapp';
import { round } from 'lodash';

import { AppState } from '../app/app-state';

function currentLapTick({ liveTelemetry }: AppState) {
    return liveTelemetry.ticks[liveTelemetry.ticks.length - 1];
}

function wallClockElapsedTime({ liveTelemetry }: AppState) {
    return liveTelemetry.wallClockTime - liveTelemetry.wallClockStartTime;
}

export const Time = () => (state: AppState) =>
    state.liveTelemetry.anyDataReceived ? (
        <div>
            <h3>
                Time:{' '}
                {round(
                    currentLapTick(state).lapData.player.current_lap_time,
                    2
                )}
            </h3>
            <h3>Wall Clock Time: {round(wallClockElapsedTime(state), 2)}</h3>
            <h3>
                Drift:{' '}
                {round(
                    wallClockElapsedTime(state) -
                        currentLapTick(state).lapData.player.current_lap_time,
                    2
                )}
            </h3>
        </div>
    ) : null;
