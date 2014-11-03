#!/bin/bash

set -e

for kata in $(ls); do
    [[ ! -d $kata ]] && continue; 
    (cd $kata && cargo update && cargo test);
done
