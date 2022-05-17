use serde_json;

use crate::endpoints::{
    AccountCallBuilder, AssetCallBuilder, ClaimableBalanceCallbuilder, LedgerCallBuilder,
    LiquidityPoolCallBuilder, OfferCallBuilder, OperationCallBuilder, PaymentCallBuilder,
    StrictReceiveCallBuilder, StrictSendCallBuilder, TradeAggregationCallBuilder, TradeCallBuilder,
    TransactionCallBuilder,
};
use crate::types::{
    Account, Asset, ClaimableBalance, FeeStats, Ledger, LiquidityPool, Offer, Operation,
    StrictPath, Transaction,
};
use crate::utils::{req, Endpoint};

#[derive(Debug)]
pub struct Server(pub String);

impl Server {
    pub fn new(network_id: String) -> Self {
        Server(network_id)
    }

    pub fn load_account(&self, account_id: &str) -> Result<Account, &str> {
        let url = format!("{}/accounts/{}", self.0, account_id);
        let resp = req(&url).unwrap();

        let parsed: Account = serde_json::from_str(&resp).unwrap();

        Ok(parsed)
    }

    pub fn accounts(&self) -> AccountCallBuilder {
        AccountCallBuilder {
            server: self,
            cursor: None,
            order: None,
            sponsor: None,
            limit: None,
            signer: None,
            liquidity_pool: None,
            asset: None,
            endpoint: Endpoint::None,
        }
    }

    pub fn load_transaction(&self, hash: &str) -> Result<Transaction, &str> {
        let url = format!("{}/transactions/{}", self.0, hash);
        let resp = req(&url).unwrap();

        let parsed: Transaction = serde_json::from_str(&resp).unwrap();

        Ok(parsed)
    }

    pub fn transactions(&self) -> TransactionCallBuilder {
        TransactionCallBuilder {
            server: self,
            cursor: None,
            order: None,
            limit: None,
            include_failed: false,
            endpoint: Endpoint::None,
        }
    }

    pub fn load_ledger(&self, sequence: u64) -> Result<Ledger, &str> {
        let url = format!("{}/ledgers/{}", self.0, sequence);
        let resp = req(&url).unwrap();

        let parsed: Ledger = serde_json::from_str(&resp).unwrap();

        Ok(parsed)
    }

    pub fn ledgers(&self) -> LedgerCallBuilder {
        LedgerCallBuilder {
            server: self,
            cursor: None,
            order: None,
            limit: None,
            endpoint: Endpoint::None,
        }
    }

    pub fn load_offer(&self, offer_id: &str) -> Result<Offer, &str> {
        let url = format!("{}/offers/{}", self.0, offer_id);
        let resp = req(&url).unwrap();

        let parsed: Offer = serde_json::from_str(&resp).unwrap();

        Ok(parsed)
    }

    pub fn offers(&self) -> OfferCallBuilder {
        OfferCallBuilder {
            server: self,
            cursor: None,
            order: None,
            limit: None,
            buying: None,
            seller: None,
            selling: None,
            sponsor: None,
            endpoint: Endpoint::None,
        }
    }

    pub fn load_operation(&self, operation_id: &str) -> Result<Operation, &str> {
        let url = format!("{}/operations/{}", self.0, operation_id);
        let resp = req(&url).unwrap();

        let parsed: Operation = serde_json::from_str(&resp).unwrap();

        Ok(parsed)
    }

    pub fn operations(&self) -> OperationCallBuilder {
        OperationCallBuilder {
            server: self,
            cursor: None,
            order: None,
            limit: None,
            endpoint: Endpoint::None,
            include_failed: false,
        }
    }

    pub fn load_liquidity_pool(&self, liquidity_pool_id: &str) -> Result<LiquidityPool, &str> {
        let url = format!("{}/liquidity_pools/{}", self.0, liquidity_pool_id);
        let resp = req(&url).unwrap();

        let parsed: LiquidityPool = serde_json::from_str(&resp).unwrap();

        Ok(parsed)
    }

    pub fn liquidity_pools(&self) -> LiquidityPoolCallBuilder {
        LiquidityPoolCallBuilder {
            server: self,
            cursor: None,
            order: None,
            limit: None,
            assets: None,
            endpoint: Endpoint::None,
        }
    }

    pub fn load_claimable_balance(
        &self,
        claimable_balance_id: &str,
    ) -> Result<ClaimableBalance, &str> {
        let url = format!("{}/claimable_balances/{}", self.0, claimable_balance_id);
        let resp = req(&url).unwrap();

        let parsed: ClaimableBalance = serde_json::from_str(&resp).unwrap();

        Ok(parsed)
    }

    pub fn claimable_balances(&self) -> ClaimableBalanceCallbuilder {
        ClaimableBalanceCallbuilder {
            server: self,
            cursor: None,
            order: None,
            limit: None,
            endpoint: Endpoint::None,
            sponsor: None,
            asset: None,
            claimant: None,
        }
    }

    /*
        pub fn trade_aggregations<'a>(&self, base: Asset, counter: Asset, resolution: String) -> TradeAggregationCallBuilder {
            TradeAggregationCallBuilder {
                server: self,
                limit: None,
                base,
                counter,
                resolution,
            }
        }

        pub fn order_books<'a>(&self, selling: Asset, buying: Asset) -> OrderBookCallBuilder {
            OrderBookCallBuilder {
                server: self,
                limit: None,
                selling: &selling,
                buying: &buying,
            }
        }

        pub fn strict_receive_paths(
            &self,
            source_account: Option<String>,
            source_assets: Option<Vec<Asset>>,
            destination_asset: Asset,
            destination_amount: String,
        ) -> StrictReceiveCallBuilder {
            StrictReceiveCallBuilder {
                server: self,
                limit: None,
                source_assets,
                source_account,
                destination_asset,
                destination_amount,
            }
        }

        pub fn strict_send_paths(
            &self,
            destinatio_account: Option<String>,
            destination_assets: Option<Vec<Asset>>,
            source_asset: Asset,
            source_amount: String,
        ) -> StrictSendCallBuilder {
            StrictSendCallBuilder {
                server: self,
                limit: None,
                destination_assets,
                destination_account,
                source_asset,
                source_amount,
            }
        }
    */

    pub fn trades(&self) -> TradeCallBuilder {
        TradeCallBuilder {
            server: self,
            cursor: None,
            order: None,
            limit: None,
            endpoint: Endpoint::None,
            asset_pair: None,
            offer: None,
            for_type: None,
        }
    }

    pub fn payments(&self) -> PaymentCallBuilder {
        PaymentCallBuilder {
            server: self,
            cursor: None,
            order: None,
            limit: None,
            endpoint: Endpoint::None,
        }
    }

    pub fn assets(&self) -> AssetCallBuilder {
        AssetCallBuilder {
            server: self,
            cursor: None,
            order: None,
            limit: None,
            asset_code: None,
            asset_issuer: None,
            endpoint: Endpoint::None,
        }
    }

    pub fn fee_stats(&self) -> Result<FeeStats, &str> {
        let url = format!("{}/fee_stats", self.0);
        let resp = req(&url).unwrap();

        let parsed: FeeStats = serde_json::from_str(&resp).unwrap();

        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoints::call_builder::CallBuilder;

    use super::*;

    #[test]
    fn test_load_account() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let tx = s
            .load_account("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM")
            .unwrap();

        assert_eq!(tx.id, tx.account_id);
    }

    #[test]
    fn test_load_transaction() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let tx = s
            .load_transaction("3389e9f0f1a65f19736cacf544c2e825313e8447f569233bb8db39aa607c8889")
            .unwrap();

        assert_eq!(tx.id, tx.hash);
    }

    #[test]
    fn test_load_ledger() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let ledger3 = s.load_ledger(3).unwrap();
        let ledger4 = s.load_ledger(4).unwrap();

        assert_eq!(ledger3.hash, ledger4.prev_hash);
    }

    #[test]
    fn test_load_fee_stats() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let _fee_stats = s.fee_stats().unwrap();
    }

    #[test]
    fn load_operation() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let op = s.load_operation("33676838572033").unwrap();

        assert_eq!(op.id, op.paging_token);
    }

    #[test]
    fn load_some_operations() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let my_account = "GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM";

        let my_ops = s
            .operations()
            .include_failed(true)
            .for_endpoint(Endpoint::Accounts(String::from(my_account)))
            .limit(2)
            .call()
            .unwrap();

        assert_eq!(my_ops._embedded.records.len(), 2);
    }

    #[test]
    fn test_load_trade() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let my_trade = s.trades().for_offer("4").limit(1).call().unwrap();

        assert_eq!("4", my_trade._embedded.records[0].base_offer_id)
    }
}
