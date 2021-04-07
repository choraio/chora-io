# chora-web

decentralized protocol research

## Environment

### Rust

Install [Rust](https://rocket.rs/v0.4/guide/getting-started/#installing-rust) - *nightly install required for rocket*

### WASM

```
cargo install wasm-bindgen-cli wasm-pack
```

```
rustup target add wasm32-unknown-unknown --toolchain nightly
```

## Development

### Build

```
scripts/build_dev.sh
```

## Production

### Build

```
scripts/build_pro.sh
```
