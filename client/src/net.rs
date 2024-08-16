//use std::io::{Result as IoResult, Read, Write};
use std::io::Result as IoResult;
use std::net::TcpStream;

/// Structure for the games communications with the server
pub struct Networker {
    stream: Option<TcpStream>,
}

impl Networker {
    pub fn new() -> Self {
        Self {
            stream: None
        }
    }

    pub fn connect(&mut self) -> IoResult<()> {
        self.stream = Some(TcpStream::connect(format!("{}:{}", env!("SERVER"), env!("PORT")))?);
        Ok(())
    }

    /*/// Make a single connection attempt to the server defined in the environment variables.
    fn attempt_connect(&self) -> Result<TcpStream, ()>{
        let result = TcpStream::connect(format!("{}:{}", env!("SERVER"), env!("PORT")));
        if let Ok(stream) = result {
            return Ok(stream);
        }
        Err(())
    }*/

    /*/// Repeatedly make connection attempts until a stream is return, blocks the current thread.
    pub fn connect(&mut self) {
        loop {
            if let Ok(stream) = self.attempt_connect() {
                self.stream = Some(stream);
                break;
            }

            thread::sleep(Duration::from_millis(1));
        }
    }*/

    /*pub fn connect_threaded(mut self) -> JoinHandle<Self> {
        thread::spawn(move || {
            self.connect();
            self
        })
    }*/
}
