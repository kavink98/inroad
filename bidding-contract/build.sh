#!/bin/sh

rustup target add wasm32-unknown-unknown
cargo build --all --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/tender_bidding.wasm ../bidding-contract-factory/build

