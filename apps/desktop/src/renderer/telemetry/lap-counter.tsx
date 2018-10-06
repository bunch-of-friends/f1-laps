import { h } from 'hyperapp';
import { AppState } from '../app/app-state';

export const LapCounter = () => ({ liveTelemetry: { currentLap }}: AppState) => (
    <h3 class="lap-counter">
        Lap: { currentLap }
    </h3>
);
