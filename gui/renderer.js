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

const execosx = require('child_process').spawn
const execwin = require('child_process').spawn

const exit = document.getElementById('exit');
const start = document.getElementById('start');


exit.addEventListener('click', function (event) {
    ipc.send("system", "1");
});

start.addEventListener('click', function (event) {

    // ./task e 11111111 0dxZNzzwEFq7PTZWWLoyLx.mp4 D:\temp
    let platform = document.getElementById("platform").value
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
        // zindex
        document.getElementById("zindex").hidden = false;
        document.getElementById("zindex").className = "zindex"

        if (os.platform() === 'win32') {

            console.log(binDir + '/task.exe' + ' ' +Key + ' '+ SelectFile+ ' ' + savedir +' ' +platform);

            let ex = execosx(binDir + '/task.exe', ['e', Key, SelectFile, savedir, platform], {shell: true});

            ex.stderr.on('data', (data) => {
                console.log(`data = ${data}`)
            })

            ex.stdout.on('data', (data) => {
                document.getElementById("zindex").hidden = true;
                document.getElementById("zindex").className = ""
                alert(`生成文件在: ${savedir}\\${data}`.replace("encrypt out file ", ""));
            });
        }
        if (os.platform() === "darwin") {
            let ex = execosx(binDir + '/task_unix', ['e', Key, SelectFile, savedir, platform])

            ex.stderr.on('data', (data) => {
                console.log(`data = ${data}`)
            })

            ex.stdout.on('data', (data) => {
                document.getElementById("zindex").hidden = true;
                document.getElementById("zindex").className = ""
                alert(`生成文件在: ${savedir}/${data}`.replace("encrypt out file ", ""));
            });
        }
        if (os.platform() === "linux") {
            let ex = execosx(binDir + '/task_linux', ['e', Key, SelectFile, savedir, platform])

            ex.stderr.on('data', (data) => {
                console.log(data)
            })

            ex.stdout.on('data', (data) => {
                document.getElementById("zindex").hidden = true;
                document.getElementById("zindex").className = ""
                alert(`生成文件在: ${savedir}/${data}`.replace("encrypt out file ", ""));
            });
        }


    } else {
        alert("请检查参数。")
    }
});


