#!/bin/bash

# remember the current directory
start_dir=$(pwd)

# iterate over all directory names provided as arguments
for dir in "$@"; do
    # check if the directory contains a Cargo.toml file
    if [ -f "$dir/Cargo.toml" ]; then
        echo "Running command in $dir"
        # change to the directory
        cd $dir
        # run your command here (replace with your actual command)
        cargo schema
        # change back to the start directory
        cd $start_dir
    else
        echo "No Cargo.toml found in $dir, skipping"
    fi
done
