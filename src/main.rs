mod endpoints;
mod utils;

use endpoints::Server;

fn main() {
    let server = Server::new(String::from("https://horizon.stellar.org"));

    let my_account = server
        .load_account("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM")
        .unwrap();

    println!("{:#?}", my_account);
}
