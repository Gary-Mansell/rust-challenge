#!/bin/sh

cargo clean && cargo build
echo "Executing..."
./target/debug/rust-examples