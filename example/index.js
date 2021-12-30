import * as wasm from "pixel-hero";

fetch("game.nes", {
    method: "GET",
    responseType: "arraybuffer"
})
.then(res => res.arrayBuffer())
.then(data => {
    const emulator = wasm.create_emulator(document.getElementById("game"), {});
    emulator.insert(new Uint8Array(data));
})
