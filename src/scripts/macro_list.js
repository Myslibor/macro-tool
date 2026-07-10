const { invoke } = window.__TAURI__.core;

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


window.addEventListener("DOMContentLoaded", () => {
    
    document.getElementById("save_button").addEventListener("click", () => {  
        saveMacro();
    });

});