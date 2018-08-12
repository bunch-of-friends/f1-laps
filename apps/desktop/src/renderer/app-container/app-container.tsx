import { AppState } from '../app-state';
import { LapCounter } from '../lap-counter/lap-counter';
import { h } from 'hyperapp';
import { TelemetryPlot } from '../chart/telemetry-plot';

export const AppContainer = (state: AppState) => {
    return (
        <div id="container">
            <div class="log">
                <LapCounter />
                <h3 class="time">Time:</h3>
                <div>
                    <h3>Stats</h3>
                    <ul class="stats">
                        <li class="fps"></li>
                        <li class="total-points"></li>
                        <li class="displayed-points"></li>
                        <li class="speed-points"></li>
                        <li class="throttle-points"></li>
                        <li class="brake-points"></li>
                        <li class="gear-points"></li>
                        <li class="steering-points"></li>
                        <li class="compressed-points"></li>
                    </ul>
                </div>
            </div>
            <TelemetryPlot key="speed" suggestedYRange={[0, 420]} data={state.liveData.speed} />
            <canvas width="1200" height="200" id="speed-plot"></canvas>
            <canvas width="1200" height="200" id="throttle-plot"></canvas>
            <canvas width="1200" height="200" id="brake-plot"></canvas>
            <canvas width="1200" height="200" id="gear-plot"></canvas>
            <canvas width="1200" height="200" id="compressed-plot"></canvas>
            <canvas width="1200" height="200" id="steering-plot"></canvas>
        </div>
    );
};
