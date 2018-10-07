import { h } from 'hyperapp';

import { LiveTelemetryTick } from 'f1-laps-js-bridge';
import { LapCounter } from './lap-counter';
import { TelemetryPlot } from './telemetry-plot';
import { Time } from './time';
import { FPSCounter } from './fps-counter';
import { AppState } from '../app/app-state';

export const TelemetryPage = () => (state: AppState) => (
    <div>
        <LapCounter />
        <Time />
        <FPSCounter />
        <TelemetryPlot
            key="speed"
            label="Speed (kph)"
            suggestedYRange={[0, 420]}
            data={state.liveTelemetry.ticks}
            pointSelector={speedSelector}
            debug={true}
        />
        <TelemetryPlot
            key="throttle"
            label="Throttle"
            suggestedYRange={[-0.05, 1.05]}
            data={state.liveTelemetry.ticks}
            pointSelector={throttleSelector}
            debug={true}
        />
        <TelemetryPlot
            key="brake"
            label="Brake"
            suggestedYRange={[-0.05, 1.05]}
            data={state.liveTelemetry.ticks}
            pointSelector={brakeSelector}
            debug={true}
        />
        <TelemetryPlot
            key="gear"
            label="Gear"
            suggestedYRange={[-0.2, 9.2]}
            data={state.liveTelemetry.ticks}
            pointSelector={gearSelector}
            debug={true}
        />
        <TelemetryPlot
            key="steering"
            label="Steering"
            suggestedYRange={[-1, 1]}
            data={state.liveTelemetry.ticks}
            pointSelector={steeringSelector}
            debug={true}
        />
    </div>
);

function speedSelector(lapTick: LiveTelemetryTick) {
    return {
        x: lapTick.lapData.player.current_lap_time,
        y: lapTick.carTelemetry.player.speed,
    };
}

function throttleSelector(lapTick: LiveTelemetryTick) {
    return {
        x: lapTick.lapData.player.current_lap_time,
        y: lapTick.carTelemetry.player.brake,
    };
}

function brakeSelector(lapTick: LiveTelemetryTick) {
    return {
        x: lapTick.lapData.player.current_lap_time,
        y: lapTick.carTelemetry.player.brake,
    };
}

function gearSelector(lapTick: LiveTelemetryTick) {
    return {
        x: lapTick.lapData.player.current_lap_time,
        y: lapTick.carTelemetry.player.gear,
    };
}

function steeringSelector(lapTick: LiveTelemetryTick) {
    return {
        x: lapTick.lapData.player.current_lap_time,
        y: lapTick.carTelemetry.player.steer,
    };
}
