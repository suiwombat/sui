#!/usr/bin/env sh
# this script will fetch cargo data at the buck root and write out a metadata file
# that we depend on for the buckify dep tool

cd ..
cargo fetch
cargo metadata --frozen --locked --offline --format-version 1 --filter-platform aarch64-apple-darwin --filter-platform aarch64-unknown-linux-gnu --filter-platform x86_64-unknown-linux-gnu > third-party/cargo_metadata.json
cd - > /dev/null
