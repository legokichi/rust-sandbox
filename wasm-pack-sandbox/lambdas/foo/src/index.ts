const wasmRust = import("wasm-rust-crate");
import * as uouo from "uouo";

export async function handler(){
    const A = await wasmRust;
    uouo.hello();
    A.start();
}

setTimeout(()=>{
    handler();
}, 1000);
