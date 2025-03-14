# Hello World NCN Program

## Core Components

1. Program State Management

    - An NCN Admin initializes the Configuration
    - The program tracks epochs, requests, and fulfillments
    - Operators register with the program

2. Epoch-Based Processing:

    - Work is organized in epochs

## Role

### NCN Admin

- Register on Jito Restaking Program
- Request mmessages with keywords

### Operator

- Register on Jito Restaking Program
- Process the admin-requested messages
- Submit signed response

## Addresses

| Network | Program         | Address                                      | Version |
| ------- | --------------- | -------------------------------------------- | ------- |
| Devnet  | Hello World NCN | ncncd27gXkYMV56EfwntDmYhH5Wzo896yTnrBbEq9xW  | 0.1.0   |

### Test

```bash
cargo-build-sbf && SBF_OUT_DIR=$(pwd)/target/sbf-solana-solana/release cargo nextest run --all-features
```

### Generate Clients

```bash
cargo b -p shank-cli && ./target/debug/shank-cli && yarn generate-clients && cargo b
```

### Run Operator Client

```bash
cargo r -p hello-world-operator-cli -- --config-path ./operator-cli/operators_config.json
```

## CLI

