#!/usr/bin/env bash

sudo xcode-select -s $1
xcode-select -p
clang --version

clang++ -O0 main.cpp -target x86_64-apple-darwin
./a.out
echo "the expected value is $?"

clang++ -O3 main.cpp -target x86_64-apple-darwin
./a.out
echo "the actual value is $? with ld-prime"

clang++ -O3 main.cpp -target x86_64-apple-darwin -Wl,-ld_classic
./a.out
echo "the actual value is $? with ld64(-ld_classic)"
