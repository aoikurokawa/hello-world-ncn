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
| Testnet | Hello World NCN | DXWJEC5JBUeNurpo7wPDUHGhDWnjkTzUiV3gp2D9y8zr | 0.1.0   |
| Devnet  | Hello World NCN | DXWJEC5JBUeNurpo7wPDUHGhDWnjkTzUiV3gp2D9y8zr | 0.1.0   |

### Test

```bash
cargo-build-sbf && SBF_OUT_DIR=$(pwd)/target/sbf-solana-solana/release cargo nextest run --all-features
```

### Generate Clients

```bash
cargo b -p shank-cli && ./target/debug/shank-cli && yarn generate-clients && cargo b
```

## CLI

### HELP!

```bash
cargo r -p cli ncn-portal whitelist --help
```

### Initialize Whitelist

```bash
cargo r -p cli ncn-portal whitelist initialize --keypair "KEYPAIR" --rpc-url "https://api.devnet.solana.com" --ncn-portal-program-id "DwyMNTQ5aSduQhx3Pjra9kXeySxjD5YUkC1bDXmvEPFZ"
```

### Add to Whitelist

```bash
cargo r -p cli ncn-portal whitelist add-to-whitelist "DyEKpfGg6sBL2Dg6rnHcsdAHJdCoe7Ur5VWzDzdHkQY6" 100 --keypair "KEYPAIR" --rpc-url "https://api.devnet.solana.com" --ncn-portal-program-id "DwyMNTQ5aSduQhx3Pjra9kXeySxjD5YUkC1bDXmvEPFZ"
```

