const { invoke } = window.__TAURI__.core;

async function renderMacroButtons(){
    let container = document.getElementById("bricks");
    let macros = await invoke("get_macros");
    console.log(JSON.stringify(macros));

    container.innerHTML = '';

    const grid = document.createElement('div');
    grid.className = 'bricks-grid';

    macros.forEach((macro, index) => {
        const button = document.createElement('button');
        button.className = 'brick-btn';

        button.addEventListener("click", async () => {
            await invoke("edit_macro", { index: index });
            console.log("entered macro editor for ",index);
            window.location.href = "edit.html";
        });

        button.textContent = `${index+1}. "${macro.name}" : ${macro.key_bind}`;
        button.dataset.index = index;

        grid.appendChild(button);
    });

    container.appendChild(grid);
}


window.addEventListener("DOMContentLoaded", () => {
    
    document.getElementById("back_button").addEventListener("click", () => {  
        window.location.href = 'index.html';
    });

    renderMacroButtons();
});