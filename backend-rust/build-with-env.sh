#!/bin/sh

# BUILD_MODE **MUST** be one of "develop" or "release"

if [ "${BUILD_MODE}" = "develop" ]
then
  echo "Building with develop mode"
  cargo build
elif [ "${BUILD_MODE}" = "release" ]
then
  echo "Building with release mode"
  cargo build --release
else
  echo "BUILD_MODE must be one of \"develop\" or \"release\"."
  exit 1
fi
