const { invoke } = window.__TAURI__.core;

let timeAfterPara;
let timeAfter = 1.0;
let keyCode = "";

async function readKey(event) {
    console.log("You pressed:", event.key);
    keyCode = event.code;
    await invoke("read_key", {keyName: event.key, keyCode: event.code});
    document.removeEventListener("keydown", read_key);
}

async function setTime() {
    let time = await timeDialog();
    await invoke("set_time", {time: time});
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


window.addEventListener("DOMContentLoaded", () => {
    document.getElementById("select_button").addEventListener("click", () => {
        document.addEventListener("keydown", read_key);
    });

    document.getElementById("time_button").addEventListener("click", () => {  
        setTime();
    });

    timeAfterPara = document.getElementById("time_after_key");
    timeAfterPara.textContent = "Time after key:" + timeAfter;

});