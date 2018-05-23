const { app, BrowserWindow } = require('electron');
const path = require('path');
const url = require('url');
const f1 = require('f1-laps-js-bridge');

console.log('node test is running');


// f1.newSession.register((data) => console.log('newSession >> ', data));
// core.liveData.register((data) => console.log('liveData >> ', data));
// core.bestSessionLap.register((data) => console.log('bestSessionLap >> ', data));
// core.bestSessionSector.register((data) => console.log('bestSessionSector >> ', data));
// core.bestEverLap.register((data) => console.log('bestEverLap >> ', data));
// f1.rawData.register((data) => {
//     console.log(
//         'track: ' + data.track_id,
//         'team: ' + data.team_id,
//         'mode: ' + data.session_type,
//         'lap: ' + data.lap,
//         'time: ' + data.lap_time,
//         'invalid: ' + data.current_lap_invalid,
//         'gear: ' + data.gear,
//         'throttle: ' + data.throttle,
//         'break: ' + data.brake,
//         'last_lap: ' + data.last_lap_time
//     );
// });

// f1.replayStoredData();

function startApp() {
    const window = new BrowserWindow({
        width: 1280,
        height: 720,
        webPreferences: {
            nodeIntegration: false,
            preload: path.join(__dirname, 'preload.js')
        }
    });

    window.loadURL(url.format({
        pathname: path.join(__dirname, 'index.html'),
        protocol: 'file:',
        slashes: true
    }));

    window.webContents.openDevTools();
}

app.on('ready', startApp);
