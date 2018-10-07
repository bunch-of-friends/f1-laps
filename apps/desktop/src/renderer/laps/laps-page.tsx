import { h } from 'hyperapp';

import { AppState } from '../app/app-state';
import { AppActions } from '../app/app-actions';

import './laps.css';
import { LapHeader } from 'f1-laps-js-bridge';

export const LapsPage = () => (state: AppState, actions: AppActions) => (
    <div class="laps-page">
        <button onclick={() => actions.getLaps()}>Load laps</button>
        <hr />
        <table id="laps">
            <thead>
                <tr>
                    <th>Era</th>
                    <th>Track ID</th>
                    <th>Session type</th>
                    <th>Tyre compound</th>
                    <th>Weather</th>
                    <th>Team ID</th>
                    <th>Lap Time</th>
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
                {state.storedLaps.map(lap => {
                    return (
                        <tr>
                            <td>{lap.era}</td>
                            <td>{lap.track_id}</td>
                            <td>{lap.session_type}</td>
                            <td>{lap.tyre_compound}</td>
                            <td>{lap.weather}</td>
                            <td>{lap.team_id}</td>
                            <td>
                                {lap.lap_time} [{lap.sector_times[0]},{' '}
                                {lap.sector_times[1]}, {lap.sector_times[2]}]
                            </td>
                            <td>
                                {renderLapReferenceButton(lap, state, actions)}
                            </td>
                            <td>
                                <button
                                    onclick={() =>
                                        actions.deleteTelemetry(lap.id)
                                    }
                                >
                                    Delete
                                </button>
                            </td>
                        </tr>
                    );
                })}
            </tbody>
        </table>
    </div>
);

function renderLapReferenceButton(
    lap: LapHeader,
    state: AppState,
    actions: AppActions
) {
    if (state.referenceLap && state.referenceLap.id === lap.id) {
        return (
            <button onclick={() => actions.unsetReferenceLap()}>Unset</button>
        );
    } else {
        return (
            <button onclick={() => actions.setReferenceLap(lap.id)}>
                Set as reference
            </button>
        );
    }
}
