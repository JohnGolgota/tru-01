const { app, BrowserWindow } = require('electron')

const createWindow = () => {
    const win = new BrowserWindow({
        width: 500,
        height: 400,
        webPreferences:{
            preload: ""
        }
    })

    win.loadFile('index.html')
}

app.whenReady().then(() => {
    createWindow()
})