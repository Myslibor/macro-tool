const { invoke } = window.__TAURI__.core;


let timeAfterPara;
let timeAfter = 1.0;

let selectedKeyPara;
let selectedKey = "KeyA";

let macro;

async function readKey(event) {
    console.log("You pressed:", event.key);
    await invoke("read_key", {keyName: event.key, keyCode: event.code});
    document.removeEventListener("keydown", readKey);

    selectedKey = event.code;
    selectedKeyPara.textContent = "Selected key: " + selectedKey;
}

async function setTime() {
    let time = await timeDialog();
    timeAfter = time;
    await invoke("set_time", {time: time});

    timeAfterPara.textContent = "Time after key: " + timeAfter;
}

function timeDialog() {
    const dialog = document.getElementById("waitDialog");
    const input = document.getElementById("waitInput");

    input.value = ""
    dialog.showModal();

    return new Promise((resolve) => {
        dialog.addEventListener("close", () => {
            if (dialog.returnValue !== "ok") {
                resolve(null);
                return;
            }
            
            const time = Number.parseFloat(input.value);
            console.log(time);

            if(Number.isNaN(time)){
                alert("Please enter a valid float number!");
                resolve(null);
                return;
            }
            resolve(time);
        },{once: true});
    });

}

async function addBrick() {
    await invoke("add_brick");
    renderBricksButtons()
}

async function renderBricksButtons(){
    let container = document.getElementById("bricks");
    let macro = await invoke("get_new_macro");
    console.log(JSON.stringify(macro));

    let bricks = macro.bricks;

    container.innerHTML = '';

    const grid = document.createElement('div');
    grid.className = 'bricks-grid';

    bricks.forEach((brick, index) => {
        const button = document.createElement('button');
        button.className = 'brick-btn';
        button.textContent = `${index+1}: ` + brick.button + ` ${brick.wait}s`;
        button.dataset.index = index;

        grid.appendChild(button);
    });

    container.appendChild(grid);
}


window.addEventListener("DOMContentLoaded", () => {
    document.getElementById("select_button").addEventListener("click", () => {
        document.addEventListener("keydown", readKey);
    });

    document.getElementById("time_button").addEventListener("click", () => {  
        setTime();
    });

    document.getElementById("add_brick").addEventListener("click", () => {  
        addBrick();
    });
    
    document.getElementById("save_button").addEventListener("click", () => {  
        
    });

    timeAfterPara = document.getElementById("time_after_key");
    timeAfterPara.textContent = "Time after key: " + timeAfter;

    selectedKeyPara = document.getElementById("selected_key");
    selectedKeyPara.textContent = "Selected key: " + selectedKey;

});