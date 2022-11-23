#!/bin/sh -l

# Add cargo binaries to PATH
export PATH=/usr/local/cargo/bin:$PATH

# execute cargo commands passed in to the `docker run [...]` command
cargo $@
