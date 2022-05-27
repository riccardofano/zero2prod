This repository was made following the [Zero To Production In Rust](https://zero2prod.com) book, an opinionated introduction to backend development using Rust, made by [Luca Palmieri](https://github.com/LukeMathWalker).
I highly recommend it!

## Pre-requisites

You'll need to install:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

A faster OS-dependent linker:

### Windows

```bash
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

### Linux

```bash
# Ubuntu
sudo apt-get install lld clang libssl-dev postgresql-client
# Arch
sudo pacman -S lld clang postgresql
```

### MacOS

```bash
brew install michaeleisel/zld/zld
```

The SQLx CLI tool:

```bash
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres
```

## How to build

Launch a Postgres database and a Redis instance via Docker:

```bash
./scripts/init_db.sh
./scripts/init_redis.sh
```

Launch `cargo`:

```bash
cargo build
```

## How to test

Launch a Postgres database and a Redis instance via Docker:

```bash
./scripts/init_db.sh
./scripts/init_redis.sh
```

Launch `cargo`:

```bash
cargo test
```
