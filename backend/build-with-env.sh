#!/bin/sh

# If BUILD_MODE is "release" build with release mode,
# otherwise build with debug mode.

if [ "${BUILD_MODE}" = "release" ]
then
  echo "Building with release mode"
  cargo build --color never --release
else
  echo "Building with debug mode"
  cargo build --color never
fi
