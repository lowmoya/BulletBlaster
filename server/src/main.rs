use std::io::{Result as IoResult, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    println!("Listening to {:#?}", stream);
    let _ = stream.set_read_timeout(None);
    let _ = stream.read(&mut [0; 128]);
}


fn main() -> IoResult<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", env!("PORT")))?;

    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(|| handle_client(stream));
    }

    Ok(())
}
