mod endpoints;
mod types;
mod utils;

use endpoints::{CallBuilder, Server};
use types::Asset;
use utils::Endpoint;

fn main() {
    let horizon = String::from("https://horizon.stellar.org");
    let server = Server::new(horizon);

    let rbt = Asset::new(
        "RBT",
        "GCMSCRWZ3QBOI6AF75B5ZWDBXOSMIRW4FSBZH5OI65Y4H4GVH7LPSOYS",
    );

    let _offers = server
        .offers()
        .limit(2)
        .for_endpoint(Endpoint::Accounts(String::from(
            "GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM",
        )))
        .selling(&rbt)
        .call();

    let an = server.transactions().limit(2).call().unwrap();

    println!("{:#?}", an);
}
