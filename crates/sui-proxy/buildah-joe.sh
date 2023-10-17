#!/usr/bin/env bash
# meh this copy should be done by buck, not a shell script
cp $1 crates/sui-proxy/
# hack to be in the right folder for the stupid dockerfile
cd crates/sui-proxy

# rm of sui_proxy should also be unnecessary
buildah build -t sui-proxy ; rm sui_proxy ; cd - > /dev/null