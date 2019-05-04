const js = import("./node_modules/zydis-wasm/zydis_wasm.js");
js.then(js => {
    js.greet("WebAssembly");
});
