import * as wasm from "pixel-hero";

fetch("game.nes", {
    method: "GET",
    responseType: "arraybuffer"
})
.then(res => res.arrayBuffer())
.then(data => {
    const wrapper = document.getElementById("game");
    const emulator = wasm.create_emulator(wrapper, {});
    
    const button = createButton(wrapper);
    button.onclick = function() {
        button.style.display = "none";
        emulator.insert(new Uint8Array(data));
    }
})

// Web audio can only be played after a user event,
// So we insert a button here and user must click it to start the game.
function createButton(wrapper) {
    const button = document.createElement("div");
    button.style.cursor = "pointer";
    button.style.backgroundImage = "url(./play.svg)";
    button.style.backgroundPosition = "center center";
    button.style.backgroundSize = "15vw 15vw";
    button.style.backgroundRepeat = "no-repeat";
    button.style.position = "absolute";
    button.style.left = 0;
    button.style.top = 0;
    document.body.appendChild(button);

    function resize() {
        button.style.width = wrapper.offsetWidth + "px";
        button.style.height = wrapper.offsetHeight + "px";
    }
    resize();
    window.addEventListener("resize", resize);

    return button;
}
