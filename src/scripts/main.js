const { invoke } = window.__TAURI__.core;

async function createNew(params){
  window.location.href = "edit.html";
}

async function saveApp(params) {
  await invoke("save_everything");
}

window.addEventListener("DOMContentLoaded", () => {

  document.getElementById("create_macros_button").addEventListener("click", () => {
    createNew();
  });
  document.getElementById("save").addEventListener("click", () => {
    saveApp();
  });
});




