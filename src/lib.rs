//! # Stellar Sdk
//!
//! A lightweight Rust library for communicating with a Stellar Horizon server.
//!
//! ## Usage:
//!
//!
//! ```
//! use stellar_sdk::{CallBuilder, Server, types::Asset, utils::{Direction, Endpoint}};
//!
//!     let s = String::from("https://horizon.stellar.org");
//!     let s = Server::new(s, None).expect("Cannot connect to insecure horizon server");
//!
//!     let my_acc = s
//!         .load_account("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM")
//!         .unwrap();
//!
//!     // Load transactions of an account
//!     let my_account_id = String::from("GAP2TJNW7NL52MPB36DZ2PB6PSIBEUEJXDG325BJQKUNDQBPKX3E2DLV");
//!     let my_txs = s
//!         .transactions()
//!         .order(Direction::Desc)
//!         .limit(2)
//!         .include_failed(false)
//!         .for_endpoint(Endpoint::Accounts(my_account_id))
//!         .call()
//!         .unwrap();
//!
//!     // Load trades of yXLM and XLM
//!     let y_xlm = Asset::new(
//!         String::from("yXLM"),
//!         String::from("GARDNV3Q7YGT4AKSDF25LT32YSCCW4EV22Y2TV3I2PU2MMXJTEDL5T55"),
//!     ).unwrap();
//!
//!     let native = Asset::native();
//!
//!     let xlm_trades = s
//!         .trades()
//!         .for_asset_pair(&y_xlm, &native)
//!         .limit(2)
//!         .call()
//!         .unwrap();
//!
//!     // Load USDC liquidity pools
//!     let usdc = Asset::new(
//!         String::from("USDC"),
//!         String::from("GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"),
//!     ).unwrap();
//!     let usdc_liquidity_pools = s.liquidity_pools().for_assets(vec![usdc]).call().unwrap();
//!
//! ```

mod api_call;
mod endpoints;
#[cfg(feature = "nacl")]
mod keypair;
mod operations;
mod str_key;
pub mod types;
pub mod utils;

pub use endpoints::CallBuilder;
pub use endpoints::Server;
pub use endpoints::StellarTomlResolver;
#[cfg(feature = "nacl")]
pub use keypair::Keypair;
pub use str_key::StrKey;

#[cfg(test)]
mod tests {
    use crate::{
        types::Asset,
        utils::{Direction, Endpoint},
        CallBuilder, Server,
    };

    #[test]
    fn test_app() {
        let s = String::from("https://horizon.stellar.org");
        let s = Server::new(s, None).expect("Cannot connect to insecure horizon server");

        let _my_acc = s
            .load_account("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM")
            .unwrap();

        // Load transactions of an account
        let my_account_id =
            String::from("GAP2TJNW7NL52MPB36DZ2PB6PSIBEUEJXDG325BJQKUNDQBPKX3E2DLV");
        let _my_txs = s
            .transactions()
            .order(Direction::Desc)
            .limit(2)
            .include_failed(false)
            .for_endpoint(Endpoint::Accounts(my_account_id))
            .call()
            .unwrap();

        // Load trades of yXLM and XLM
        let y_xlm = Asset::new(
            String::from("yXLM"),
            String::from("GARDNV3Q7YGT4AKSDF25LT32YSCCW4EV22Y2TV3I2PU2MMXJTEDL5T55"),
        )
        .unwrap();

        let native = Asset::native();

        let _xlm_trades = s
            .trades()
            .for_asset_pair(&y_xlm, &native)
            .limit(2)
            .call()
            .unwrap();

        // Load USDC liquidity pools
        let usdc = Asset::new(
            String::from("USDC"),
            String::from("GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"),
        )
        .unwrap();
        let _usdc_liquidity_pools = s.liquidity_pools().for_assets(vec![usdc]).call().unwrap();
    }
}
