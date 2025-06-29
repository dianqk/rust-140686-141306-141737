#!/usr/bin/env bash

export MACOSX_DEPLOYMENT_TARGET="10.12"
xcode-select -p
xcode-select -v
clang --version
rustc -vV

RUSTFLAGS="-Clink-arg=-ld_classic" cargo build --release --target x86_64-apple-darwin

echo "Using -ld_classic"
nc -l localhost 8888 &
./target/x86_64-apple-darwin/release/origin

cargo build --release --target x86_64-apple-darwin
echo "Not using -ld_classic"
nc -l localhost 8888 &
./target/x86_64-apple-darwin/release/origin

rustc -Copt-level=3 -Ccodegen-units=256 -Clink-arg=-ld_classic --target x86_64-apple-darwin main.rs && ./main
