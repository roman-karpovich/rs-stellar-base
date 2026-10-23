# Stellar Base Library

![Crates.io](https://img.shields.io/crates/v/stellar-baselib)
![Crates.io](https://img.shields.io/crates/l/stellar-baselib)
![Crates.io](https://img.shields.io/crates/d/stellar-baselib)

A library that offers a comprehensive set of functions for reading, writing, hashing, and signing primitive XDR constructs utilized in the Stellar network. it provides a nice abstraction for building and signing transactions

Supports Protocol 30 XDR for Soroban and Stellar classic operations. This branch
prepares the unreleased 0.7.0 compatibility update.

## Quickstart

Add this to your Cargo.toml:

```toml
[dependencies]
stellar-baselib = "0.7.0"
```

And this to your code:

```rust
use stellar_baselib::*;
```

## Protocol 30 migration

- Requires Rust 1.87 or newer; uses `stellar-xdr 30` and `stellar-strkey 1`.
- CAP-88: ledger headers support `SignedMs` and `EmptyTxSetMs`. The existing
  `close_time` field and transaction time bounds remain in whole seconds.
- CAP-84: `Address` accepts W-addresses, preserves their unsigned 64-bit ID,
  and converts them to and from `ScAddress::MuxedContract` and `ScVal`.
  `Address::muxed_contract` accepts 40 bytes: contract ID, then big-endian ID.
  W-addresses are transfer destinations, not contracts to invoke or auth signers.
  A contract's `Address` argument or storage key still requires a plain address.
- CAP-87 cost types and the 1024-byte contract-spec type/event names are
  available through the re-exported `xdr` module.
- XDR types from older major versions are distinct Rust types. Consumers must
  update their dependency graph together rather than add a second XDR version.

The 0.7.0 quickstart becomes available on crates.io after the release. Before
then, use a Git dependency pinned to the reviewed commit.

## How to run tests

```bash
cargo test
```

## Coding Best Practices Used

1. All Rust code is linted with Clippy with the command `cargo clippy`. If preferred to ignore its advice, do so explicitly:
   `#[allow(clippy::too_many_arguments)]`

2. All rust code is formatted with `cargo fmt`. rustfmt.toml defines the expected format.

3. Function and local variable names follow snake_case. Structs or Enums follow CamelCase and Constants have all capital letters.
