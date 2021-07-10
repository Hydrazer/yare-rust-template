const toml = require("toml");
const fs = require("fs");
const { execSync } = require("child_process");

let cargoProjectName = toml
  .parse(fs.readFileSync("./Cargo.toml", "utf-8"))
  .package.name.replace(/\-/g, "_");

let script = `node wasm2js/wasm2yareio.js target/wasm32-unknown-unknown/release/${cargoProjectName}.wasm --no-auto-update`;

console.log(script);

let out = execSync(script);

console.log(out.toString());
