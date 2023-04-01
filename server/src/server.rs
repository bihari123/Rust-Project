use std::{io::Read, net::TcpListener};

#[derive(Debug)]
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }

    pub fn run(&self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            let res = listener.accept();

            match listener.accept() {
                Ok((mut stream, socketAdd)) => {
                    println!("{:?}, {}", stream, socketAdd);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));
                        }
                        Err(_) => {
                            println!("failed to read from the connection")
                        }
                    }
                }
                Err(e) => println!("Failed to establish a connection: {} ", e),
            }
        }
    }
}
