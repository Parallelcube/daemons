#!/bin/bash
echo Cpp service compile
gcc --version
pushd ../rust
rustc src/main.rs --out-dir build
popd