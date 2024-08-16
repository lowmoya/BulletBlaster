use std::process;
/*use std::thread::JoinHandle;*/

mod net;

use net::Networker;

struct App;

impl App {
    fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window {
    }
}

fn main() {
    let mut net = Networker::new();
    match net.connect() {
        Ok(()) => println!("Connected"),
        Err(message) => {
            eprintln!("{}", message);
            process::exit(1);
        }
    }
}
