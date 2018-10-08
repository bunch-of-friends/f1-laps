import { h } from 'hyperapp';
import { AppState } from './app-state';
import { AppActions } from './app-actions';

export const LogsView = () => (state: AppState, actions: AppActions) => {
    return (
        <div id="logs" onload={updateScroll()}>
            {state.logs.map(log => (
                <p>
                    {log.event}: {log.message}
                </p>
            ))}
        </div>
    );
};

function updateScroll() {
    const element = document.getElementById('logs');
    if (element) {
        element.scrollTop = element.scrollHeight;
    }
}
