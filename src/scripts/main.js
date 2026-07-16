const { invoke } = window.__TAURI__.core;

async function createNew(){
  await invoke("create_new_macro");
  window.location.href = "edit.html";
}

async function saveApp() {
  await invoke("save_everything");
}

window.addEventListener("DOMContentLoaded", () => {

  document.getElementById("create_macros_button").addEventListener("click", () => {
    createNew();
  });

  document.getElementById("macros_list_button").addEventListener("click", async () => {
    let macros = await invoke("get_macros");
    console.log(JSON.stringify(macros));
    window.location.href = "macro_list.html";
  });

  document.getElementById("save").addEventListener("click", () => {
    saveApp();
  });
});




