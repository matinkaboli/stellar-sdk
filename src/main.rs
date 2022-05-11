mod endpoints;
mod utils;

use endpoints::{call_builder::CallBuilder, Server};

fn main() {
    let horizon = String::from("https://horizon.stellar.org");
    let server = Server::new(horizon);

    let txs = server
        .transactions()
        .limit(3)
        .for_endpoint(utils::Endpoint::Accounts(String::from(
            "GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM",
        )))
        .call()
        .unwrap();

    println!("{:#?}", txs)
}
