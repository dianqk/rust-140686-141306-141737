#!/usr/bin/env bash

sudo xcode-select -s $1
xcode-select -p
pushd minimum
cargo clean
cargo run --target=x86_64-apple-darwin --release
popd
