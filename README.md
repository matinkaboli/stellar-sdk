# Stellar SDK

A lightweight Rust library for communicating with a Stellar Horizon server.

## Features:

- It uses only 3 dependencies and is extremely lightweight
- A networking layer API for Horizon endpoints.
- Facilities for building and signing transactions, for communicating with a Stellar Horizon instance, and for submitting transactions or querying network history.

## Install

Add `stellar_sdk` to your Cargo.toml file and run `cargo run`

```
"stellar_sdk" = "0.1.5"
```

## Usage

The usage is aimed to be identical to the [JavaScript Stellar Sdk](https://github.com/stellar/js-stellar-sdk) and we tried to avoid any complications that rust language itself has.

```rust
use stellar_sdk::{
    types::Asset,
    utils::{Direction, Endpoint},
    CallBuilder, Server,
};

fn main() {
    let s = String::from("https://horizon.stellar.org");
    let s = Server::new(s);

    // Load 1 account
    let my_acc = s
        .load_account("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM")
        .unwrap();

    // Load transactions of an account
    let my_account_id = String::from("GAP2TJNW7NL52MPB36DZ2PB6PSIBEUEJXDG325BJQKUNDQBPKX3E2DLV");
    let my_txs = s
        .transactions()
        .order(Direction::Desc)
        .limit(2)
        .include_failed(false)
        .for_endpoint(Endpoint::Accounts(my_account_id))
        .call()
        .unwrap();

    // Load trades of yXLM and XLM
    let y_xlm = Asset::new(
        "yXLM",
        "GARDNV3Q7YGT4AKSDF25LT32YSCCW4EV22Y2TV3I2PU2MMXJTEDL5T55",
    );
    let native = Asset::native();
    let my_asset_pair = (y_xlm, native);
    let xlm_trades = s
        .trades()
        .for_asset_pair(my_asset_pair)
        .limit(2)
        .call()
        .unwrap();

    // Load USDC liquidity pools
    let usdc = Asset::new(
        "USDC",
        "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
    );
    let usdc_liquidity_pools = s.liquidity_pools().for_assets(vec![usdc]).call().unwrap();

    println!("{:#?}", my_txs);
    println!("{:#?}", xlm_trades);
    println!("{:#?}", my_acc);
    println!("{:#?}", usdc_liquidity_pools);
}
```

## Todo

### Asset

- get_raw_asset_type
- to_change_trust_XDR_object
- to_trustline_XDR_object
- to_XDR_object
- from_operation

### Claimant

### FeeBumpTransaction

### Keypair

- master
- xdr_account_id
- xdr_public_key
- xdr_muxed_account
- signature_hint
- sign_payload_decorated
- sign_decorated

### LiquidityPoolAsset

### LiquidityPoolId

### Memo

### MuxedAccount

### Operation

### Server

- checkMemoRequired

### SignerKey

### Transaction

### Transaction Builder

### Globals
