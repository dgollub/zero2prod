# Zero To Production In Rust

This is my implementation of the project from [Zero To Production In Rust](https://www.zero2prod.com/).

# Prerequisites

* `cargo install cargo-watch` : automatic rebuilds on changes
* `cargo install cargo-audit` : dependencies audit
* `cargo install tarpaulin` : code coverage
* `cargo install sea-orm-cli` : SeaORM CLI
* `rustup add component clippy` : linter
* `rustup add component rustfmt` : code formatting

_Optional_

* `cargo install cargo-expand` : expand Rust macros
* `rustup toolchain install nightly --allow-downgrade` : needed for `cargo +nightly expand`

Depending on your environment you will also need to install this

## Windows

* `cargo install -f cargo-binutils`
* `rustup component add llvm-tools-preview`

## Linux

* Ubuntu: `sudo apt-get install lld clang`
* Arch: `sudo pacman -S lld clang`

## macOS

* brew install michaeleisel/zld/zld, optional, needs XCode and needs to be enabled in .cargo/config.toml

# Build instructions

* `cargo watch -x check -x test -x run`


# Differences between the book and this implementation

* [axum](https://github.com/tokio-rs/axum) instead of [actix-web](https://actix.rs/)
* `/healthcheck` endpoint instead of `/health_check`
* Use [SeaORM](https://github.com/SeaQL/sea-orm) instead of [sqlx](https://github.com/launchbadge/sqlx)

# Copyright

Copyright 2023 (c) by Daniel Kurashige-Gollub

# License

[MIT](license.md)
