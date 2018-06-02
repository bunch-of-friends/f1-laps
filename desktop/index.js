const { app, BrowserWindow } = require('electron');
const path = require('path');
const url = require('url');
const f1 = require('f1-laps-js-bridge');

console.log('node test is running');

// f1.replayAllLaps();

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
