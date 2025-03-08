FROM rust:latest as builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev libudev-dev cmake build-essential git && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Create a new empty project
WORKDIR /usr/src/app

COPY . .

# Build the project
RUN cargo build --release --bin operator-cli
RUN cargo install --path ./operator-cli --bin operator-cli --locked

# Create a runtime image with newer glibc
FROM ubuntu:22.04

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev libudev-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/operator-cli /usr/local/bin/operator-cli

COPY --from=builder /usr/src/app/operator-cli/operators_config.json /usr/local/bin/operators_config.json

COPY --from=builder /usr/src/app/credentials /usr/local/bin/

# Set the entrypoint
ENTRYPOINT ["operator-cli", "--config-path", "/app/operators_config.json"]
