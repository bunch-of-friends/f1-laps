import { h } from 'hyperapp';

import { AppState } from '../app/app-state';
import { AppActions } from '../app/app-actions';

import './laps.css';

export const LapsPage = () => (state: AppState, actions: AppActions) => (
    <div class="laps-page">
        <button onclick={() => actions.getLaps()}>Load all laps</button>
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
                    <th></th>
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
                            <td>{lap.lap_time} [{lap.sector_times[0]}, {lap.sector_times[1]}, {lap.sector_times[2]}]</td>
                            <td><button>Load telemetry</button></td>
                            <td><button>Delete</button></td>
                        </tr>
                    );
                })}
            </tbody>
        </table>
    </div>
);
