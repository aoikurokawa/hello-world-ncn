#! /bin/bash

# Function to print command being executed
print_executing() {
	echo "Executing $1"
}

print_executing "cargo nextest run --all-features"
cargo build-sbf --sbf-out-dir integration_tests/tests/fixtures
SBF_OUT_DIR=integration_tests/tests/fixtures cargo nextest run --all-features
