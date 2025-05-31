#!/usr/bin/env bash

sudo xcode-select -s $1
xcode-select -p
pushd ring
cargo clean
cargo test --target=x86_64-apple-darwin --release -p ring --test hmac_tests
popd
