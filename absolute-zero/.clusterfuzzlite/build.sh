#!/bin/bash -eu
# SPDX-License-Identifier: MPL-2.0
cd "$SRC"/absolute-zero
cargo +nightly fuzz build
# cargo-fuzz writes binaries to `fuzz/target/<triple>/release/<target>`
# (the fuzz crate's own target dir), not to the workspace root. The
# previous `./target/...` path was a vestige of an earlier layout where
# fuzz/ depended on the parent crate's `[lib]` (since removed).
for target in $(cargo +nightly fuzz list); do
    cp fuzz/target/x86_64-unknown-linux-gnu/release/$target $OUT/
done
