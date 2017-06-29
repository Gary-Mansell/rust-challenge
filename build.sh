#!/bin/sh -e

# SPEED > binary compatibility!!!
# cargo clean && 
cargo build
echo "Executing..."
./target/debug/rust-challenge