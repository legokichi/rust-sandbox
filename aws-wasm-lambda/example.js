/*
cargo build &&
wasm-pack build -t nodejs &&
wasm-pack pack &&
node example.js
*/
const pkg = require("./pkg");

async function main () {
    await pkg.handler({
        sleep(ms){ return new Promise(resolve=> setTimeout(resolve, ms)); },
        count: 3,
        wait: 1000
    });
    await pkg.handler2({
        sleep(ms){ return new Promise(resolve=> setTimeout(resolve, ms)); },
        periodic(ms, cb){ let i = 0; let tid = setInterval(() => { cb(i++); }, ms); return ()=> clearInterval(tid); },
    });
}

main().then(console.log).catch(console.error);