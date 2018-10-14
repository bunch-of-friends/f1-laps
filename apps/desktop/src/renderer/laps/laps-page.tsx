import { h } from 'hyperapp';

import { AppState } from '../app/app-state';
import { AppActions } from '../app/app-actions';
import { nanosecondsToString, secondsToString, idToString } from '../helpers/formatting';

import './laps.css';
import { LapHeader } from 'f1-laps-js-bridge';

export const LapsPage = () => (state: AppState, actions: AppActions) => (
    <div class="laps-page">
        <button onclick={() => actions.getLaps()}>Load laps</button>
        <hr />
        <table id="laps">
            <thead>
                <tr>
                    <th>Recored</th>
                    <th>Era</th>
                    <th>Track</th>
                    <th>Session type</th>
                    <th>Tyre compound</th>
                    <th>Weather</th>
                    <th>Team</th>
                    <th>Lap Time</th>
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
                {state.storedLaps.map(lap => {
                    return (
                        <tr>
                            <td>
                                {nanosecondsToString(
                                    lap.recorded_date,
                                    'Do MMM YY HH:mm'
                                )}
                            </td>
                            <td>{idToString('era', lap.era)}</td>
                            <td>{idToString('track_id', lap.track_id)}</td>
                            <td>
                                {idToString('session_type', lap.session_type)}
                            </td>
                            <td>
                                {idToString('tyre_compound', lap.tyre_compound)}
                            </td>
                            <td>{idToString('weather', lap.weather)}</td>
                            <td>{idToString('team_id', lap.team_id)}</td>
                            <td>
                                {secondsToString(lap.lap_time)} [
                                {secondsToString(lap.sector_times[0])},{' '}
                                {secondsToString(lap.sector_times[1])},{' '}
                                {secondsToString(lap.sector_times[2])}]
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
