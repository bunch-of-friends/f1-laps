import { h } from 'hyperapp';
import { AppState } from '../app-state';

export const LapCounter = () => ({ liveData: { currentLap }}: AppState) => (
    <h3 class="lap-counter">
        Lap: { currentLap }
    </h3>
);
