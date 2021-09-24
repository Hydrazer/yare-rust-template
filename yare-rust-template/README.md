# rust-yare-template

A easy to setup template for ViliamVadocz/yare-rust

## Known issues

None!

## Setup

Dependencies:

- rustup (https://rustup.rs/)
- node.js v16 (https://nodejs.org/en/)
- yarn (`npm i -g yarn` when node is installed)

Installation steps:

1. `git clone https://github.com/swz-gh/rust-yare-template.git`
2. `cd rust-yare-template`
3. `yarn`
4. You're done!

You can now edit your code in `src/lib.rs` and check out the wrappers in `src/wrappers.rs`.

## Building

Building is as simple as `yarn build`. Once its done you will have a file called `bundle.js`.
If you wanna upload it to yare, run `yarn upload` and it'll compile and upload it to your active games.
