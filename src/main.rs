mod server;
mod method;
mod query;

fn main() {
    let server = server::Server::new("0.0.0.0", "3000");
    server.listen();
}
