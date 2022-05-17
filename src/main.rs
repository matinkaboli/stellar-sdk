use stellar_sdk::endpoints::Server;

fn main() {
    let s = String::from("https://horizon.stellar.org");
    let s = Server::new(s);

    let acc = s
        .load_account("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM")
        .unwrap();

    println!("{:#?}", acc);
}
