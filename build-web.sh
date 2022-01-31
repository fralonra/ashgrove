#!/bin/sh

cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --out-dir ./public/pkg/ --target web ./target/wasm32-unknown-unknown/release/ashgrove.wasm

cp -r ./assets ./public