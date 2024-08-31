//mod net;

use std::process;

use client::Client;
//use net::Networker;

fn main() {
    let _client = Client::new().unwrap_or_else(|err| {
        eprintln!("Failed to run app: {err}");
        process::exit(1);
    });

    /*let mut net = Networker::new();
    match net.connect() {
        Ok(()) => println!("Connected"),
        Err(message) => {
            eprintln!("{}", message);
            process::exit(1);
        }
    }*/
}
