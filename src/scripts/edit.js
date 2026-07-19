const { invoke } = window.__TAURI__.core;


let timeAfterPara = document.getElementById("time_after_key");
let timeAfter = 1.0;

let selectedKeyPara = document.getElementById("selected_key");
let selectedKey = "KeyA";

let macroNamePara = document.getElementById("macro_name");
let macroName;

let keyBindPara = document.getElementById("macro_keybind");
let keyBind;

let hasLoopPara = document.getElementById("has_loop");
let hasLoop;

let macro;

async function load_new_data() {
    macroName = await invoke("get_new_name");
    keyBind = await invoke("get_key_bind");
    hasLoop = await invoke("get_new_has_loop");
}

async function readKey(event) {
    console.log("You pressed:", event.key);
    await invoke("read_key", {keyName: event.key, keyCode: event.code});
    document.removeEventListener("keydown", readKey);

    selectedKey = event.code;
    selectedKeyPara.textContent = "Selected key: " + selectedKey;
}

async function readKeyBind(event) {
    if(event.code == 'Escape'){
        document.removeEventListener("keydown", readKeyBind);
        console.log("Final keyBind: ", keyBind);

        await invoke("set_key_bind", {keyBind: keyBind})

        keyBind = [];
        let temp_keybind = await invoke("get_key_bind");
        keyBindPara.textContent = "KeyBind is: " + temp_keybind;
        return;
    }

    if(!keyBind.includes(event.code)){
        keyBind.push(event.code);
    }
    
    console.log("current key bind: ", keyBind);
}

async function setName() {
    let name = await window.prompt("Input this macro's name: ", "A name");

    if(name == null){
        alert("Please enter a valid name!");
        return;
    }

    macroName = name;
    await invoke("set_new_name", {name: name});

    macroNamePara.textContent = "Macro name: " + name;
}

async function setTime() {
    let input = await window.prompt("Enter the time to wait for:","1.0");

    const time = Number.parseFloat(input);
    console.log(time);

    if(Number.isNaN(time)){
        alert("Please enter a valid float number!");
        resolve(null);
        return;
    }

    timeAfter = time;
    await invoke("set_time", {time: time});

    timeAfterPara.textContent = "Time after key: " + timeAfter;
}

async function addBrick() {
    await invoke("add_brick");
    renderBricksButtons();
}

async function changeLoop() {
    hasLoop = !hasLoop;
    hasLoopPara.textContent = "Has loop: " + hasLoop;
    await invoke("set_loop",{hasLoop: hasLoop});
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

        button.addEventListener("click", async () => {
            await invoke("delete_brick", { index: index });
            console.log("deleted brick nr.",index);
            renderBricksButtons();
        });

        button.textContent = `${index+1}: ` + brick.button + ` ${brick.wait}s`;
        button.dataset.index = index;

        grid.appendChild(button);
    });

    container.appendChild(grid);
}

async function saveMacro(){
    let is_saved = await invoke("save_macro");

    if(is_saved == false ){
        alert("Can't save a macro with no bricks, no name or no key bind!");
        return;
    }

    window.location.href = "index.html";
}


window.addEventListener("DOMContentLoaded", async () => {

    await load_new_data();

    timeAfterPara.textContent = "Time after key: " + timeAfter;
    selectedKeyPara.textContent = "Selected key: " + selectedKey;
    macroNamePara.textContent = "Macro name: " + macroName;
    keyBindPara.textContent = "KeyBind is: " + keyBind;
    hasLoopPara.textContent = "Has loop: " + hasLoop;

    document.getElementById("select_button").addEventListener("click", () => {
        document.addEventListener("keydown", readKey);
    });

    document.getElementById("time_button").addEventListener("click", () => {  
        setTime();
    });

    document.getElementById("change_name_button").addEventListener("click", () => {  
        setName();
    });

    document.getElementById("select_keybind_button").addEventListener("click", () => {
        document.addEventListener("keydown", readKeyBind);
        keyBind = [];
    });

    document.getElementById("loop_button").addEventListener("click", () => {  
        changeLoop();
    });

    document.getElementById("add_brick").addEventListener("click", () => {  
        addBrick();
    });
    
    document.getElementById("save_button").addEventListener("click", () => {  
        saveMacro();
    });

    document.getElementById("abort_button").addEventListener("click", () => {  
        window.location.href = "index.html";
    });

    renderBricksButtons();

});