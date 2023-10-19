use std::io::Read;
use std::net::TcpListener;
use crate::request::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(interface: &str, port: &str) -> Self {
        let addr = format!("{}:{}", interface, port);
        println!("Listening on {}", addr);
        Server { addr }
    }

    pub fn listen(self) {
        loop {
            let listener = TcpListener::bind(&self.addr).unwrap();

            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf: [u8; 1024] = [0; 1024];

                    match stream.read(&mut buf) {
                        Ok(_) => {
                            let request = String::from_utf8_lossy(&buf).to_string();

                            match Request::try_from(request) {
                                Ok(r) => print!(
                                    "path: {}, method: {:?}, protocol: {}",
                                    r.path, r.method, r.protocol
                                ),
                                Err(e) => println!("Error {:?}", e),
                            };
                        }
                        Err(e) => println!("Error reading from stream: {}", e),
                    }
                }
                Err(e) => println!("Error accepting connection: {}", e),
            }
        }
    }
}
