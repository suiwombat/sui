#!/usr/bin/env sh

# Check if exactly one argument is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 folder name here"
    exit 1
fi

folder="$1"

mkdir fixups/$folder
touch fixups/$folder/fixups.toml

