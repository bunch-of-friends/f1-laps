import { h } from 'hyperapp';
import { LiveTelemetryTick } from 'f1-laps-js-bridge';

import { AppState } from '../app-state';
import { LapCounter } from '../lap-counter/lap-counter';
import { TelemetryPlot } from '../chart/telemetry-plot';
import { Time } from '../timing/time';
import { FPSCounter } from '../debug/fps-counter';

function speedSelector(lapTick: LiveTelemetryTick) {
    return {
        x: lapTick.lapData.player.current_lap_time,
        y: lapTick.carTelemetry.player.speed
    };
}

function throttleSelector(lapTick: LiveTelemetryTick) {
    return {
        x: lapTick.lapData.player.current_lap_time,
        y: lapTick.carTelemetry.player.brake
    };
}

function brakeSelector(lapTick: LiveTelemetryTick) {
    return {
        x: lapTick.lapData.player.current_lap_time,
        y: lapTick.carTelemetry.player.brake
    };
}

function gearSelector(lapTick: LiveTelemetryTick) {
    return {
        x: lapTick.lapData.player.current_lap_time,
        y: lapTick.carTelemetry.player.gear
    };
}

function steeringSelector(lapTick: LiveTelemetryTick) {
    return {
        x: lapTick.lapData.player.current_lap_time,
        y: lapTick.carTelemetry.player.steer
    };
}


export const AppContainer = (state: AppState) => {
    return (
        <div id="container">
            <LapCounter />
            <Time />
            <FPSCounter />
            <TelemetryPlot key="speed" label="Speed (kph)" suggestedYRange={[0, 420]} data={state.liveData.lapTicks} pointSelector={speedSelector} debug={true} />
            <TelemetryPlot key="throttle" label="Throttle" suggestedYRange={[-0.05, 1.05]} data={state.liveData.lapTicks} pointSelector={throttleSelector} debug={true} />
            <TelemetryPlot key="brake" label="Brake" suggestedYRange={[-0.05, 1.05]} data={state.liveData.lapTicks} pointSelector={brakeSelector} debug={true} />
            <TelemetryPlot key="gear" label="Gear" suggestedYRange={[-0.2, 9.2]} data={state.liveData.lapTicks} pointSelector={gearSelector} debug={true} />
            <TelemetryPlot key="steering" label="Steering" suggestedYRange={[-1, 1]} data={state.liveData.lapTicks} pointSelector={steeringSelector} debug={true} />
        </div>
    );
};
