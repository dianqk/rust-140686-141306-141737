#!/usr/bin/env bash

xcode-select -p
xcode-select -v
clang --version
rustc -vV

RUSTFLAGS="-Clink-arg=-ld_classic" cargo build --release --target x86_64-apple-darwin --target-dir ld_classic
cargo build --release --target x86_64-apple-darwin --target-dir no_ld_classic

echo "Using -ld_classic"
nc -l localhost 8888 &
./ld_classic/x86_64-apple-darwin/release/origin

echo "Not using -ld_classic"
nc -l localhost 8888 &
./no_ld_classic/x86_64-apple-darwin/release/origin

# rustc -Copt-level=3 -Ccodegen-units=256 -Clink-arg=-ld_classic --target x86_64-apple-darwin main.rs && ./main
