#!/bin/bash
echo Cpp service compile
gcc --version
pushd ../rust
cargo build
popd