use anyhow::anyhow;
use chrono::prelude::*;
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
    StrictPathSource, SubmitTransactionResponse, Transaction,
};
use crate::utils::request::get_current_server_time;

use super::EffectCallBuilder;

use stellar_base::{transaction::Transaction as TransactionSBase, xdr::XDRSerialize};

#[derive(Debug, Clone)]
pub struct Server {
    pub server_url: String,
    pub options: ServerOptions,
    pub timebounds: Option<Timebounds>,
    pub submit_transaction_options: Option<SubmitTransactionOptions>,
}

#[derive(Debug, Clone)]
pub struct ServerOptions {
    pub allow_http: Option<bool>,
    pub app_name: Option<String>,
    pub app_version: Option<String>,
    pub auth_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Timebounds {
    pub min_time: i64,
    pub max_time: i64,
}

#[derive(Debug, Clone)]
pub struct SubmitTransactionOptions {
    pub skip_memo_required_check: Option<bool>,
}

impl Server {
    pub fn new(server_url: String, opts: Option<ServerOptions>) -> Result<Self, anyhow::Error> {
        // If the opts parameter provided we are unwrapping, if not then we are giving default value
        let options = opts.unwrap_or_else(|| ServerOptions {
            allow_http: Some(false),
            app_name: None,
            app_version: None,
            auth_token: None,
        });

        // Non https not allowed in production
        if &server_url.trim()[..5] != "https" && !options.allow_http.unwrap() {
            return Err(anyhow!("Cannot connect to insecure horizon server"));
        }

        Ok(Server {
            server_url,
            options: options,
            timebounds: None,
            submit_transaction_options: None,
        })
    }

    pub fn set_auth_token(&mut self, token: String) {
        self.options.auth_token = Option::from(token);
    }

    pub fn load_account(&self, account_id: &str) -> Result<Account, anyhow::Error> {
        let url = format!("{}/accounts/{}", self.server_url, account_id);
        api_call::<Account>(
            url,
            crate::types::HttpMethod::GET,
            &HashMap::new(),
            &self.options.auth_token,
        )
    }

    pub fn accounts(&self) -> AccountCallBuilder {
        AccountCallBuilder::new(self)
    }

    pub fn load_transaction(&self, hash: &str) -> Result<Transaction, anyhow::Error> {
        let url = format!("{}/transactions/{}", self.server_url, hash);
        api_call::<Transaction>(
            url,
            crate::types::HttpMethod::GET,
            &HashMap::new(),
            &self.options.auth_token,
        )
    }

    pub fn transactions(&self) -> TransactionCallBuilder {
        TransactionCallBuilder::new(self)
    }

    pub fn load_ledger(&self, sequence: u64) -> Result<Ledger, anyhow::Error> {
        let url = format!("{}/ledgers/{}", self.server_url, sequence);
        api_call::<Ledger>(
            url,
            crate::types::HttpMethod::GET,
            &HashMap::new(),
            &self.options.auth_token,
        )
    }

    pub fn ledgers(&self) -> LedgerCallBuilder {
        LedgerCallBuilder::new(self)
    }

    pub fn load_offer(&self, offer_id: &str) -> Result<Offer, anyhow::Error> {
        let url = format!("{}/offers/{}", self.server_url, offer_id);
        api_call::<Offer>(
            url,
            crate::types::HttpMethod::GET,
            &HashMap::new(),
            &self.options.auth_token,
        )
    }

    pub fn offers(&self) -> OfferCallBuilder {
        OfferCallBuilder::new(self)
    }

    pub fn load_operation(&self, operation_id: &str) -> Result<Operation, anyhow::Error> {
        let url = format!("{}/operations/{}", self.server_url, operation_id);
        api_call::<Operation>(
            url,
            crate::types::HttpMethod::GET,
            &HashMap::new(),
            &self.options.auth_token,
        )
    }

    pub fn operations(&self) -> OperationCallBuilder {
        OperationCallBuilder::new(self)
    }

    pub fn load_liquidity_pool(
        &self,
        liquidity_pool_id: &str,
    ) -> Result<LiquidityPool, anyhow::Error> {
        let url = format!("{}/liquidity_pools/{}", self.server_url, liquidity_pool_id);
        api_call::<LiquidityPool>(
            url,
            crate::types::HttpMethod::GET,
            &HashMap::new(),
            &self.options.auth_token,
        )
    }

    pub fn liquidity_pools(&self) -> LiquidityPoolCallBuilder {
        LiquidityPoolCallBuilder::new(self)
    }

    pub fn load_claimable_balance(
        &self,
        claimable_balance_id: &str,
    ) -> Result<ClaimableBalance, anyhow::Error> {
        let url = format!(
            "{}/claimable_balances/{}",
            self.server_url, claimable_balance_id
        );
        api_call::<ClaimableBalance>(
            url,
            crate::types::HttpMethod::GET,
            &HashMap::new(),
            &self.options.auth_token,
        )
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
        let url = format!("{}/fee_stats", self.server_url);
        api_call::<FeeStats>(
            url,
            crate::types::HttpMethod::GET,
            &HashMap::new(),
            &self.options.auth_token,
        )
    }

    pub fn fetch_base_fee(&self) -> Result<String, anyhow::Error> {
        let fee_stats = self.fee_stats()?;
        let base_fee = fee_stats.last_ledger_base_fee;
        Ok(base_fee)
    }

    pub fn fetch_timebounds(
        &mut self,
        seconds: i64,
        is_retry: bool,
    ) -> Result<Timebounds, anyhow::Error> {
        let current_server_time = get_current_server_time(&self.server_url);

        if !current_server_time.is_none() && is_retry == false {
            Ok(Timebounds {
                min_time: 0,
                max_time: current_server_time.unwrap() + seconds,
            })
        } else if is_retry == true {
            let local_now: DateTime<Local> = Local::now();
            let local_timestamp = local_now.timestamp();
            Ok(Timebounds {
                min_time: 0,
                max_time: local_timestamp + seconds,
            })
        } else {
            self.fetch_timebounds(seconds, true)
        }
    }

    pub fn submit_transaction(
        &self,
        transaction: TransactionSBase,
    ) -> Result<SubmitTransactionResponse, anyhow::Error> {
        let tx = transaction.into_envelope().xdr_base64()?;
        let url = format!("{}/transactions", self.server_url);

        let mut query = HashMap::new();
        query.insert("tx".to_string(), tx.to_string());

        api_call::<SubmitTransactionResponse>(
            url,
            crate::types::HttpMethod::POST,
            &query,
            &self.options.auth_token,
        )
    }

    pub fn effects(&self) -> EffectCallBuilder {
        EffectCallBuilder::new(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{endpoints::call_builder::CallBuilder, utils::Endpoint};
    use std::str::FromStr;
    use stellar_base::{
        amount::Amount,
        asset::Asset,
        memo::Memo,
        operations::Operation,
        transaction::{Transaction, MIN_BASE_FEE},
        crypto::SodiumKeyPair, Network, PublicKey,
    };

    use super::*;

    #[test]
    fn test_load_account() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let tx = s
            .load_account("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM")
            .unwrap();

        assert_eq!(tx.id, tx.account_id);
    }

    #[test]
    fn test_load_transaction() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let tx = s
            .load_transaction("3389e9f0f1a65f19736cacf544c2e825313e8447f569233bb8db39aa607c8889")
            .unwrap();

        assert_eq!(tx.id, tx.hash);
    }

    #[test]
    fn test_load_ledger() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let ledger3 = s.load_ledger(3).unwrap();
        let ledger4 = s.load_ledger(4).unwrap();

        assert_eq!(ledger3.hash, ledger4.prev_hash);
    }

    #[test]
    fn test_load_fee_stats() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let _fee_stats = s.fee_stats().unwrap();
    }

    #[test]
    fn test_load_base_fee() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let _base_fee = s.fetch_base_fee().unwrap();
    }

    #[test]
    fn load_operation() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let op = s.load_operation("33676838572033").unwrap();

        assert_eq!(op.id, op.paging_token);
    }

    #[test]
    fn load_some_operations() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

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
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let my_trade = s.trades().for_offer("4").limit(1).call().unwrap();

        assert_eq!("4", my_trade._embedded.records[0].base_offer_id)
    }

    #[test]
    fn test_fetch_timebounds() {
        let mut s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let timebounds = s.fetch_timebounds(10000, false).unwrap();
        let local_now: DateTime<Local> = Local::now();
        let local_timestamp = local_now.timestamp();
        assert!(timebounds.min_time + local_timestamp < timebounds.max_time);
    }

    #[test]
    fn test_submit_transaction() {
        let s = Server::new(String::from("https://horizon-testnet.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        // Test can easily fail because someone drained the wallet, but it's okay for now later can be used .env or always asking the friendbot with new random wallet
        let source_keypair =
        SodiumKeyPair::from_secret_seed("SCPQMOR2R2PGTFGBHXTSP4KB47Y6XVLAZEOCCMSAU6QXP3KPLXRVXZBV")
                .unwrap();

        let destination =
            PublicKey::from_account_id("GAST24JSPH5S5Z2HC5PKEVQYDZIPFLOEC26KLVDNPVFVNNRALVTM6SCN")
                .unwrap();

        let payment_amount = Amount::from_str("0.1").unwrap();

        let payment = Operation::new_payment()
            .with_destination(destination.clone())
            .with_amount(payment_amount)
            .unwrap()
            .with_asset(Asset::new_native())
            .build()
            .unwrap();

        let account = s
            .load_account(&source_keypair.public_key().clone().to_string())
            .unwrap();

        let sequence = account.sequence.parse::<i64>().unwrap() + 1;

        let mut tx =
            Transaction::builder(source_keypair.public_key().clone(), sequence, MIN_BASE_FEE)
                .with_memo(Memo::Text("stellar_sdk_test".to_string()))
                .add_operation(payment)
                .into_transaction()
                .unwrap();

        let _ = tx.sign(&source_keypair.as_ref(), &Network::new_test());

        let response = s.submit_transaction(tx);
        assert_eq!(response.is_ok(), true);
    }
}
