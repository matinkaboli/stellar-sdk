use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{
    AccountCallBuilder, AssetCallBuilder, ClaimableBalanceCallbuilder, LedgerCallBuilder,
    LiquidityPoolCallBuilder, OfferCallBuilder, OperationCallBuilder, OrderBookCallBuilder,
    PaymentCallBuilder, StrictReceiveCallBuilder, StrictSendCallBuilder,
    TradeAggregationCallBuilder, TradeCallBuilder, TransactionCallBuilder,
};
use crate::types::{
    Account, Asset, ClaimableBalance, FeeStats, Ledger, LiquidityPool, Offer, Operation,
    StrictPathSource, Transaction,
};

#[derive(Debug)]
pub struct Server(pub String);

impl Server {
    pub fn new(network_id: String) -> Self {
        Server(network_id)
    }

    pub fn load_account(&self, account_id: &str) -> Result<Account, anyhow::Error> {
        let url = format!("{}/accounts/{}", self.0, account_id);
        api_call::<Account>(url, crate::types::HttpMethod::GET, &HashMap::new())
    }

    pub fn accounts(&self) -> AccountCallBuilder {
        AccountCallBuilder::new(self)
    }

    pub fn load_transaction(&self, hash: &str) -> Result<Transaction, anyhow::Error> {
        let url = format!("{}/transactions/{}", self.0, hash);
        api_call::<Transaction>(url, crate::types::HttpMethod::GET, &HashMap::new())
    }

    pub fn transactions(&self) -> TransactionCallBuilder {
        TransactionCallBuilder::new(self)
    }

    pub fn load_ledger(&self, sequence: u64) -> Result<Ledger, anyhow::Error> {
        let url = format!("{}/ledgers/{}", self.0, sequence);
        api_call::<Ledger>(url, crate::types::HttpMethod::GET, &HashMap::new())
    }

    pub fn ledgers(&self) -> LedgerCallBuilder {
        LedgerCallBuilder::new(self)
    }

    pub fn load_offer(&self, offer_id: &str) -> Result<Offer, anyhow::Error> {
        let url = format!("{}/offers/{}", self.0, offer_id);
        api_call::<Offer>(url, crate::types::HttpMethod::GET, &HashMap::new())
    }

    pub fn offers(&self) -> OfferCallBuilder {
        OfferCallBuilder::new(self)
    }

    pub fn load_operation(&self, operation_id: &str) -> Result<Operation, anyhow::Error> {
        let url = format!("{}/operations/{}", self.0, operation_id);
        api_call::<Operation>(url, crate::types::HttpMethod::GET, &HashMap::new())
    }

    pub fn operations(&self) -> OperationCallBuilder {
        OperationCallBuilder::new(self)
    }

    pub fn load_liquidity_pool(
        &self,
        liquidity_pool_id: &str,
    ) -> Result<LiquidityPool, anyhow::Error> {
        let url = format!("{}/liquidity_pools/{}", self.0, liquidity_pool_id);
        api_call::<LiquidityPool>(url, crate::types::HttpMethod::GET, &HashMap::new())
    }

    pub fn liquidity_pools(&self) -> LiquidityPoolCallBuilder {
        LiquidityPoolCallBuilder::new(self)
    }

    pub fn load_claimable_balance(
        &self,
        claimable_balance_id: &str,
    ) -> Result<ClaimableBalance, anyhow::Error> {
        let url = format!("{}/claimable_balances/{}", self.0, claimable_balance_id);
        api_call::<ClaimableBalance>(url, crate::types::HttpMethod::GET, &HashMap::new())
    }

    pub fn claimable_balances(&self) -> ClaimableBalanceCallbuilder {
        ClaimableBalanceCallbuilder::new(self)
    }

    pub fn trade_aggregations<'a>(
        &'a self,
        base: &'a Asset,
        counter: &'a Asset,
        resolution: &'a str,
    ) -> TradeAggregationCallBuilder {
        TradeAggregationCallBuilder::new(self, base, counter, resolution)
    }

    pub fn order_books(&self, selling: Asset, buying: Asset) -> OrderBookCallBuilder {
        OrderBookCallBuilder::new(self, selling, buying)
    }

    pub fn strict_receive_paths<'a>(
        &'a self,
        source: &StrictPathSource,
        destination_asset: Asset,
        destination_amount: String,
    ) -> StrictReceiveCallBuilder {
        StrictReceiveCallBuilder::new(self, source, &destination_asset, &destination_amount)
    }

    pub fn strict_send_paths<'a>(
        &'a self,
        destination: &StrictPathSource,
        source_asset: &'a Asset,
        source_amount: &'a str,
    ) -> StrictSendCallBuilder {
        StrictSendCallBuilder::new(self, destination, source_asset, source_amount)
    }

    pub fn trades(&self) -> TradeCallBuilder {
        TradeCallBuilder::new(self)
    }

    pub fn payments(&self) -> PaymentCallBuilder {
        PaymentCallBuilder::new(self)
    }

    pub fn assets(&self) -> AssetCallBuilder {
        AssetCallBuilder::new(self)
    }

    pub fn fee_stats(&self) -> Result<FeeStats, anyhow::Error> {
        let url = format!("{}/fee_stats", self.0);
        api_call::<FeeStats>(url, crate::types::HttpMethod::GET, &HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::{endpoints::call_builder::CallBuilder, utils::Endpoint};

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
