#!/usr/bin/env zsh
cd vendor
for dir in $(ls -d */ | sed 's/\/$//'); do echo "Build //third-party:$dir"; buck2 build //third-party:$dir#check ||  { echo "Command failed for directory: $dir"; }; done && echo "Perhaps it is not all ruined?"

cd - > /dev/null
