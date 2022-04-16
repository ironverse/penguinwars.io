# penguinwars.io

## What is Penguin Wars?

Penguin Wars is a decentralized MMO-RTS game. (Massively Multiplayer Real-time Strategy) 
- It is built by and for Penguin Finance and Cryptopuffies communities on the Avalanche blockchain. 
- It uses existing blockchain assets like PEFI and Cryptopuffy NFTs.
- It is built in Rust and compiled to WASM which runs in the browser.
- The game runs peer-to-peer through Ironverse which is a decentralized game hub.
- We aim to be maximally decentralized and transparent. We keep everything open and publicly visible.

## Game Features

Unchecked means not yet implemented.

- [ ] Protect your penguin emperor with your puffies. If your penguin dies, your kingdom is destroyed.
- [ ] Order your puffies to move around the map and fight other puffies (whether they are staked or not)
- [ ] Use puffies to gather resources
- [ ] Use resources to build structures
- [ ] Destroy enemy structures to steal their resources
- [ ] Team up with other players by creating or joining a faction
- [ ] Rent out your puffies to earn revenue while they aren't in use. Don't have a puffy? Rent one instead of buying one.

## Project Setup

```
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

## Compile

```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name penguinwars-io --out-dir assets --target web target/wasm32-unknown-unknown/release/penguinwars-io.wasm
```

## Run

```
# cargo install basic-http-server
basic-http-server .
```


Join us on Discord: https://discord.gg/AwRQGhvfnh