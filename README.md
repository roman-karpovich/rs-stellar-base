# Stellar Base Library

A library that offers a comprehensive set of functions for reading, writing, hashing, and signing primitive XDR constructs utilized in the Stellar network. it provides a nice abstraction for building and signing transactions

## Coding Best Practices Used

1. All Rust code is linted with Clippy with the command `cargo clippy`. If preferred to ignore its advice, do so explicitly:
   `#[allow(clippy::too_many_arguments)]`

2. All rust code is formatted with `cargo fmt`. rustfmt.toml defines the expected format.

3. Function and local variable names follow snake_case. Structs or Enums follow CamelCase and Constants have all capital letters.
