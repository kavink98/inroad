#!/bin/sh

rustup target add wasm32-unknown-unknown
cargo build --package project --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/project.wasm ./build
cargo build --package project-factory --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/project_factory.wasm ./build