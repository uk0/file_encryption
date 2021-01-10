// auth zhangjianxin
const {app, BrowserWindow} = require('electron')

const dialog = require('electron').dialog;
const ipc = require('electron').ipcMain
const os = require('os')


const path = require('path')

function createWindow() {
    // Create the browser window.
    const mainWindow = new BrowserWindow({
        width: 420,
        height: 640,
        disableAutoHideCursor:true,
        webPreferences: {
            preload: path.join(__dirname, 'preload.js'),
            enableRemoteModule: true,
            nodeIntegration: true
        },
        icon: path.join(__dirname, 'images/logo.png')
    })

    // and load the index.html of the app.
    mainWindow.loadFile('index.html')

    // mainWindow.webContents.openDevTools();

    if (process.platform === 'darwin') {
        app.dock.setIcon(path.join(__dirname, 'images/logo.png'));
    }


    ipc.on("system",async event=>{
        app.quit()
    });

    ipc.on('open-file-dialog-for-file', async event => {
        if (os.platform() === 'linux' || os.platform() === 'win32') {
            dialog.showOpenDialog({
                properties: ['openFile']
            }).then(result => {
                console.log(result.filePaths[0]);
                if (result) event.sender.send('selected-file', result.filePaths[0]);
            }).catch(err => {
                console.log(err)
            })

        } else {
            dialog.showOpenDialog({
                properties: ['openFile', 'openDirectory']
            }).then(result => {
                console.log(result.filePaths[0]);
                if (result) event.sender.send('selected-file', result.filePaths[0]);

            }).catch(err => {
                console.log(err)
            })
        }
    });


}

app.whenReady().then(() => {
    createWindow()

    app.on('activate', function () {
        if (BrowserWindow.getAllWindows().length === 0) createWindow()
    })
})


// explicitly with Cmd + Q.
app.on('window-all-closed', function () {
    if (process.platform !== 'darwin') app.quit()
})
