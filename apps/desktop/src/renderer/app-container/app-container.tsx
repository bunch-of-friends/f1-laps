import { AppState } from '../app-state';
import { LapCounter } from '../lap-counter/lap-counter';
import { h } from 'hyperapp';
import { TelemetryPlot } from '../chart/telemetry-plot';
import { LapTick } from 'f1-laps-js-bridge';
import { Time } from '../timing/time';
import { FPSCounter } from '../debug/fps-counter';

function speedSelector(lapTick: LapTick) {
    return {
        x: lapTick.currentLapTime,
        y: lapTick.currentSpeed
    };
}

function throttleSelector(lapTick: LapTick) {
    return {
        x: lapTick.currentLapTime,
        y: lapTick.throttle
    };
}

function brakeSelector(lapTick: LapTick) {
    return {
        x: lapTick.currentLapTime,
        y: lapTick.brake
    };
}

function gearSelector(lapTick: LapTick) {
    return {
        x: lapTick.currentLapTime,
        y: lapTick.currentGear
    };
}

function steeringSelector(lapTick: LapTick) {
    return {
        x: lapTick.currentLapTime,
        y: lapTick.steer
    };
}


export const AppContainer = (state: AppState) => {
    return (
        <div id="container">
            <LapCounter />
            <Time />
            <FPSCounter />
            <TelemetryPlot key="speed" label="Speed (kph)" suggestedYRange={[0, 420]} data={state.liveData.lapTicks} pointSelector={speedSelector} debug={true}/>
            <TelemetryPlot key="throttle" label="Throttle" suggestedYRange={[-0.05, 1.05]} data={state.liveData.lapTicks} pointSelector={throttleSelector} debug={true}/>
            <TelemetryPlot key="brake" label="Brake" suggestedYRange={[-0.05, 1.05]} data={state.liveData.lapTicks} pointSelector={brakeSelector} debug={true}/>
            <TelemetryPlot key="gear" label="Gear" suggestedYRange={[-0.2, 9.2]} data={state.liveData.lapTicks} pointSelector={gearSelector} debug={true}/>
            <TelemetryPlot key="steering" label="Steering" suggestedYRange={[-1, 1]} data={state.liveData.lapTicks} pointSelector={steeringSelector} debug={true}/>
        </div>
    );
};
