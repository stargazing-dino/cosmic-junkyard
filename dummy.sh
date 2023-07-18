#!/bin/bash

# Concatenate all files in /src directory into a single mod.rs file in the root directory
find ./src -type f -name '*.rs' ! -name 'mod.rs' -exec cat {} + > ./mod.rs

# Run format command on mod.rs file
rustfmt ./mod.rs

# Raname file to mod.txt
mv ./mod.rs ./mod.txt