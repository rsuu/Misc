#!/bin/bash

cd ./script && bash gen_lib.sh
cd ..

cargo build
mv /tmp/cargo/target/debug/test_script ./

# run
LD_LIBRARY_PATH=~/.local/share/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib ./test_script

# rustc +nightly -Cprefer-dynamic -Z print-link-args hello.rs
