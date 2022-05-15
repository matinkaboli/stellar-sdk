mod endpoints;
mod utils;

use endpoints::{call_builder::CallBuilder, Server};

fn main() {
    let horizon = String::from("https://horizon.stellar.org");
    let server = Server::new(horizon);

    let txs = server.load_offer("2").unwrap();

    println!("{:#?}", txs)
}
