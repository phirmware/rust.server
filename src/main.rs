use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;
use std::str::FromStr;

fn main() {
    let server = Server::new("0.0.0.0", "3000");
    server.listen();
}

struct Server {
    addr: String,
}

impl Server {
    pub fn new(interface: &str, port: &str) -> Self {
        let addr = format!("{}:{}", interface, port);
        println!("Listening on {}", addr);
        Server { addr }
    }

    fn listen(self) {
        loop {
            let listener = TcpListener::bind(&self.addr).unwrap();
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf: [u8; 246] = [0; 246];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            let request = String::from_utf8_lossy(&buf).to_string();
                            println!("Request Data: {}", request);
                            match Request::try_from(request) {
                                Ok(r) => print!(
                                    "path: {}, protocol: {}", r.path, r.protocol
                                ),
                                Err(e) => println!("Invalid request {}", e),
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

// POST /health?pickaboo=bamm HTTP/1.1
struct Request {
    method: Method,
    path: String,
    protocol: String,
}

impl TryFrom<String> for Request {
    type Error = String;

    fn try_from(request: String) -> Result<Self, Self::Error> {
        let (method, request) = get_request_segment(&request).ok_or("Invalid Request")?;
        let method = Method::from_str(method).or(Err("Invalid Method"))?;

        let (path, request) = get_request_segment(&request).ok_or("Invalid Request")?;
        let (protocol, _request) = get_request_segment(&request).ok_or("Invalid Request")?;

        Ok(Request {
            method: method,
            path: path.to_string(),
            protocol: protocol.to_string(),
        })
    }
}

enum Method {
    GET,
    HEAD,
    POST,
    PATCH,
    PUT,
    DELETE,
    CONNECT,
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PATCH" => Ok(Method::PATCH),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "CONNECT" => Ok(Method::CONNECT),

            _ => Err("Invalid method".to_string()),
        }
    }
}

fn get_request_segment(request_line: &str) -> Option<(&str, &str)> {
    match request_line.find(" ") {
        Some(space_index) => {
            let segment = &request_line[..space_index];
            // using (space_index+1) cause space is a one byte character
            let rest = &request_line[(space_index + 1)..];

            return Some((segment, rest));
        }
        None => return None,
    }
}
