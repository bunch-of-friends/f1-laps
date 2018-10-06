import { h } from 'hyperapp';
import { round } from 'lodash';

import { AppState } from '../app/app-state';

export const FPSCounter = () => ({ liveTelemetry: { fps } }: AppState) => (
    <div>FPS: {isNaN(fps.currentFPS) ? '' : round(fps.currentFPS)}</div>
);
