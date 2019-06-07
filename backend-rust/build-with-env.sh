#!/bin/sh

# MODE **MUST** be one of "develop" or "production"

if [ "${MODE}" = "develop" ]
then
  echo "Building with develop mode"
  cargo build
elif [ "${MODE}" = "production" ]
then
  echo "Building with production mode"
  cargo build --release
else
  echo "MODE must be one of \"develop\" or \"production\"."
  exit 1
fi
