#!/bin/sh -e

cargo clean && cargo build
echo "Executing..."
./target/debug/rust-challenge