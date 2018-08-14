import { h } from 'hyperapp';
import { round } from 'lodash';

import { AppState } from '../app-state';

export const FPSCounter = () => ({ liveData: { fps } }: AppState) => (
    <div>FPS: {isNaN(fps.currentFPS) ? '' : round(fps.currentFPS)}</div>
);
