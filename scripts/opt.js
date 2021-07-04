const toml = require("toml");
const fs = require("fs");
const { execSync } = require("child_process");

let cargoProjectName = toml
  .parse(fs.readFileSync("./Cargo.toml", "utf-8"))
  .package.name.replace(/\-/g, "_");

let command = `wasm-opt -O4 target/wasm32-unknown-unknown/release/${cargoProjectName}.wasm -o target/wasm32-unknown-unknown/release/${cargoProjectName}-opt.wasm`;
console.log(command);

let out = execSync(command);

console.log(out.toString());
