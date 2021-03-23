#!/bin/bash
set -eu


# FOLDER_NAME=${PWD##*/}
# CRATE_NAME=$FOLDER_NAME # assume crate name is the same as the folder name
EXAMPLE_NAME="demo"

rustup target add wasm32-unknown-unknown

# Release:
cargo build --release --example ${EXAMPLE_NAME} --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/examples/${EXAMPLE_NAME}.wasm docs/

# # Debug:
# cargo build --example ${EXAMPLE_NAME} --target wasm32-unknown-unknown
# cp target/wasm32-unknown-unknown/debug/examples/${EXAMPLE_NAME}.wasm docs/
