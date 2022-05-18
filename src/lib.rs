//! # Stellar Sdk
//!
//! A lightweight Rust library for communicating with a Stellar Horizon server.
//!
//! ## Usage:
//!
//! ```
//! use stellar_sdk::{
//!     types::Asset,
//!     utils::{Direction, Endpoint},
//!     CallBuilder, Server,
//! };
//!
//! fn main() {
//!     let s = String::from("https://horizon.stellar.org");
//!     let s = Server::new(s);
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
//!         "yXLM",
//!         "GARDNV3Q7YGT4AKSDF25LT32YSCCW4EV22Y2TV3I2PU2MMXJTEDL5T55",
//!     );
//!
//!     let native = Asset::native();
//!     let my_asset_pair = (y_xlm, native);
//!     let xlm_trades = s
//!         .trades()
//!         .for_asset_pair(my_asset_pair)
//!         .limit(2)
//!         .call()
//!         .unwrap();
//!
//!     // Load USDC liquidity pools
//!     let usdc = Asset::new(
//!         "USDC",
//!         "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
//!     );
//!     let usdc_liquidity_pools = s.liquidity_pools().for_assets(vec![usdc]).call().unwrap();
//!
//!     println!("{:#?}", my_txs);
//!     println!("{:#?}", xlm_trades);
//!     println!("{:#?}", my_acc);
//!     println!("{:#?}", usdc_liquidity_pools);
//! }
//! ```

pub mod endpoints;
pub mod types;
pub mod utils;

pub use endpoints::CallBuilder;
pub use endpoints::Server;
