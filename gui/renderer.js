const ipc = require('electron').ipcRenderer;
const selectFileBtn = document.getElementById('select-directory');
const os = require("os")
let isDebug = false;

let SelectFile = null;

let Key = null;
ipc.on('selected-file', function (event, path) {
    SelectFile = path;
});

selectFileBtn.addEventListener('click', function (event) {
    ipc.send('open-file-dialog-for-file')
});

const exec = require('child_process').spawn

const exit = document.getElementById('exit');
const start = document.getElementById('start');

exit.addEventListener('click', function (event) {
    ipc.send("system", "1");
});

start.addEventListener('click', function (event) {

    // ./task e 11111111 0dxZNzzwEFq7PTZWWLoyLx.mp4 D:\temp
    let entry = document.getElementById("entry").value
    let key1 = document.getElementById("key1").value
    let key2 = document.getElementById("key2").value
    let savedir = document.getElementById("savedir").value

    let binDir = "";
    if (key1 == key2 && SelectFile != null && savedir != null) {
        if (isDebug) {
            binDir = "./bin"
        } else {
            binDir = process.resourcesPath + "/app/bin"
        }

        Key = key1
        if (os.platform() === 'win32') {
            exec(binDir + '/task.exe', ['e', Key, SelectFile, savedir]).stdout.on('data', (data) => {
                alert(`生成文件在: ${__dirname}/${data}`.replace("encrypt out file ", ""));
            });
        }
        exec(binDir + '/task', ['e', Key, SelectFile, savedir]).stdout.on('data', (data) => {
            alert(`生成文件在: ${data}`.replace("encrypt out file ", ""));
        });

    } else {
        alert("请检查参数。")
    }
});


