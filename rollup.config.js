import { terser } from "rollup-plugin-terser";
import { babel } from "@rollup/plugin-babel";
import toml from "toml";
import fs from "fs";

let cargoProjectName = toml
  .parse(fs.readFileSync("./Cargo.toml", "utf-8"))
  .package.name.replace(/\-/g, "_");

export default {
  input: `target/wasm32-unknown-unknown/release/${cargoProjectName}-opt.js`,
  output: {
    file: "bundle.js",
    format: "esm",
  },
  plugins: [babel({ babelHelpers: "bundled" }), terser()],
};
