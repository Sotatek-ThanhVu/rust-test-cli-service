# Project name

CLI Service for Blockchain Balance

## Description

This is a simple Rust CLI project that fetch user balance in a specific timestamp on BSC Chain.

## Requirements

- Rust >= 1.79
- BSC API Key. You can archive api key [here](https://bscscan.com/myapikey)

## How to run

- You have to create a `.env` first. You can look over `.env.example` for the sample environment variables.
- Build the project

```sh
cargo build
```

- And run

```sh
./target/debug/rev-cli -a <YOUR ADDRESS> -t <TIMESTAMP>
```

Or you can run it directly

```sh
cargo run -- -a <YOUR ADDRESS> -t <TIMESTAMP>
```
