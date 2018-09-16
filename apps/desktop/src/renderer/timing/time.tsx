import { h } from 'hyperapp';
import { round } from 'lodash';

import { AppState } from '../app-state';

function currentLapTick({ liveData }: AppState) {
    return liveData.lapTicks[liveData.lapTicks.length - 1];
}

function wallClockElapsedTime({ liveData }: AppState) {
    return liveData.wallClockTime - liveData.wallClockStartTime
}

export const Time = () => (state: AppState) => (
    state.liveData.anyDataReceived ?
        <div>
            <h3>Time: {round(currentLapTick(state).lapData.player.current_lap_time, 2)}</h3>
            <h3>Wall Clock Time: {round(wallClockElapsedTime(state), 2)}</h3>
            <h3>Drift: {round(
                wallClockElapsedTime(state) - currentLapTick(state).lapData.player.current_lap_time,
                2
            )}</h3>
        </div> : null
);
