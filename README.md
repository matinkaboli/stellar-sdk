# Stellar SDK

A Rust library for communicating with a Stellar Horizon server.

It provides:

    - a networking layer API for Horizon endpoints.

    - facilities for building and signing transactions, for communicating with a Stellar Horizon instance, and for submitting transactions or querying network history.

## Install

Add `stellar_sdk` to your Cargo.toml file and run `cargo run`

```
"stellar_sdk" = "0.1.0"
```

## Usage

```rust
use stellar_sdk::endpoints::Server;

fn main() {
    let s = String::from("https://horizon.stellar.org");
    let s = Server::new(s);

    let acc = s
        .load_account("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM")
        .unwrap();

    println!("{:#?}", acc);
}
```
