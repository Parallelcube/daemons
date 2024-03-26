#!/bin/bash
echo Cpp service compile
gcc --version
pushd ../cpp
make clean
make
popd