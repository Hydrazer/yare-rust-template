const { readdirSync } = require("fs");
const { execSync } = require("child_process");

let wasmfile = readdirSync("./target/wasm32-unknown-unknown/release/").find(
  (a) => a.endsWith(".wasm")
);

let command = `yareio-wasm-gluer ./target/wasm32-unknown-unknown/release/${wasmfile}`;

console.log(execSync(command).toString());
