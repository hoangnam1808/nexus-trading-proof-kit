# Nexus Trading Proof Kit

A small builder-path repo for Nexus Testnet III.

This project demonstrates a simple verifiable trading-journal computation using the Nexus zkVM:

- Private input: encoded trade outcomes in half-R units.
- Public input: expected public journal score.
- Public output: verified score if the private data matches the public summary.

The goal is not to promise profit or run a trading bot. The goal is to show a realistic use case for verifiable computation: a trader can prove that a journal summary was computed correctly without revealing every raw trade detail.

## Why this fits Nexus

Nexus zkVM proves program execution. A trading journal is full of small but important calculations: net R, win rate, risk-used, fee-adjusted result, and drawdown. This repo starts with a minimal net-R proof so it can be expanded later.

## Official setup flow

Install Rust first, then run the official Nexus zkVM setup commands:

```bash
rustup target add riscv32i-unknown-none-elf
rustup run nightly-2025-05-09 cargo install --git https://github.com/nexus-xyz/nexus-zkvm cargo-nexus --tag 'v0.3.6'
rustup run nightly-2025-05-09 cargo nexus --help
```

Generate a host project:

```bash
rustup run nightly-2025-05-09 cargo nexus host nexus-host
```

Then replace these generated files with the versions in this repo:

```text
nexus-host/src/main.rs
nexus-host/src/guest/src/main.rs
```

Run:

```bash
cd nexus-host
cargo run -r
```

Expected result: the host compiles the guest program, proves execution, verifies the proof, and prints a success message.

## Demo encoding

The demo encodes 8 trade outcomes in half-R units:

```text
+1.5R, -1.0R, +2.0R, -0.5R, +1.0R, 0R, -1.0R, +1.5R
```

Net result:

```text
+3.5R = 7 half-R units
score = 128 + 7 = 135
```

The public verifier only needs to know the expected score and public output. The encoded trade list is private input to the prover.

## Builder path checklist

- [ ] Run Nexus OS / web prover and capture screenshot of NEX points.
- [ ] Run Nexus CLI linked to the same Nexus account.
- [ ] Run this zkVM proof demo locally.
- [ ] Add terminal output screenshot to `proofs/`.
- [ ] Post GitHub repo + short builder note in Nexus Discord/X.
- [ ] Open a small GitHub issue or discussion if anything in docs is unclear.

## Suggested future upgrades

1. Add win rate proof.
2. Add max drawdown proof.
3. Add fee-adjusted net money proof.
4. Add JSON-to-encoded-trades helper.
5. Connect output format to a future Decision Journal export.

## Safety notes

- Do not upload private keys, seed phrases, or wallet secrets.
- Do not claim token allocation is guaranteed.
- Testnet points and tokens are for testing and can change.
