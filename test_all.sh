#!/bin/bash

set -e

for kata in $(ls); do
    [[ -d $kata ]] && (cd $kata && cargo test);
done
