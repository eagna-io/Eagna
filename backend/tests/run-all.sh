#!/bin/bash

cd $(dirname $0)

for s in $(ls scenarios); do
  ./run.sh scenarios/$s
done
