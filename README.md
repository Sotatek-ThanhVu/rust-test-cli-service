# Project name

CLI Service for Blockchain Balance

## Description

This is a simple Rust CLI project that fetch user balance in a specific timestamp on Injective Protocol

## Requirements

- Rust >= 1.79

## How to run

- You have to create a `.env` first. You can look over `.env.example` for the sample environment variables.
- Build and run the project

```sh
# Normal build
cargo build
./target/debug/rev-cli -a <YOUR ADDRESS> -t <TIMESTAMP>

# Release build
cargo build --release
./target/release/rev-cli -a <YOUR ADDRESS> -t <TIMESTAMP>
```

Or you can run it directly

```sh
cargo run -- -a <YOUR ADDRESS> -t <TIMESTAMP>
```

> [!WARNING]
> The time required to fetch transaction history will depend on the service quality of the Injective scan API.
