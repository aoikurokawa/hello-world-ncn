#! /bin/bash
set -e

# Function to print command being executed
print_executing() {
	echo "Executing $1"
}

# Basic commands that always run
print_executing "cargo sort --workspace"
cargo sort --workspace

print_executing "cargo clippy --all-features"
cargo clippy --all-features -- -D warnings -D clippy::all -D clippy::nursery -D clippy::integer_division -D clippy::arithmetic_side_effects -D clippy::style -D clippy::perf


print_executing "cargo b && ./target/debug/jito-tip-router-shank-cli && yarn install && yarn generate-clients && cargo b"
cargo b && ./target/debug/shank-cli && yarn install && yarn generate-clients && cargo b

print_executing "cargo-build-sbf"
cargo-build-sbf

print_executing "cargo nextest run --all-features"
cargo-build-sbf && SBF_OUT_DIR=$(pwd)/target/sbf-solana-solana/release cargo nextest run --all-features

