#!/bin/bash

RUSTFLAGS="-C prefer-dynamic \
    -C target-feature=-crt-static \
    " cargo build

mv /tmp/cargo/target/debug/libscript.so ../lib64.so
