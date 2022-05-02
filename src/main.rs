mod horizon;

use horizon::*;

fn main() {
    // let server = Server::new(String::from("https://horizon.stellar.org"));

    // let mut accounts = server.accounts();
    // let accounts = accounts
    //     .liquidity_pool("a468d41d8e9b8f3c7209651608b74b7db7ac9952dcae0cdf24871d1d9c7b0088")
    //     .limit(3);

    // let my_records = accounts.call();

    let rbt = Asset::new(
        "RBT",
        "GCMSCRWZ3QBOI6AF75B5ZWDBXOSMIRW4FSBZH5OI65Y4H4GVH7LPSOYS",
    );

    let lsp = Asset::new(
        "LSP",
        "GAB7STHVD5BDH3EEYXPI3OM7PCS4V443PYB5FNT6CFGJVPDLMKDM24WK",
    );

    let xlm = Asset::native();

    println!("{:#?}", rbt == xlm)
}
