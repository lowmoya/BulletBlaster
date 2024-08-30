mod net;

use std::process;

use client::App;
use net::Networker;

fn main() {
    let _app = App::new().unwrap_or_else(|err| {
        eprintln!("Failed to run app: {err}");
        process::exit(1);
    });

    let mut net = Networker::new();
    match net.connect() {
        Ok(()) => println!("Connected"),
        Err(message) => {
            eprintln!("{}", message);
            process::exit(1);
        }
    }
}
