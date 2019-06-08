#!/bin/sh

# If BUILD_MODE is "release" build with release mode,
# otherwise build with debug mode.

if [ "${BUILD_MODE}" = "release" ]
then
  echo "Building with release mode"
  cargo build --release
else
  echo "Building with develop mode"
  cargo build
fi
