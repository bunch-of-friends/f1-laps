const { app, BrowserWindow } = require('electron');
const path = require('path');
const url = require('url');

const isDevelopment = process.env.NODE_ENV !== 'production';

function startApp() {
    const window = new BrowserWindow({
        width: 1280,
        height: 720,
        webPreferences: {
            preload: path.resolve(__dirname, 'preload.js')
        }
    });

    if (isDevelopment) {
        window.loadURL(`http://localhost:${process.env.ELECTRON_WEBPACK_WDS_PORT}`);
        window.webContents.openDevTools();
    } else {
        window.loadURL(url.format({
            pathname: path.resolve(__dirname, '../renderer/index.html'),
            protocol: 'file',
            slashes: true
        }));
    }
}

app.on('ready', startApp);
