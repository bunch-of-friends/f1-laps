import { h } from 'hyperapp';

import { AppState } from '../app/app-state';

export const LapsPage = () => (state: AppState) => (
    <div class="laps-page">
        <button>Load all laps</button>
    </div>
);
