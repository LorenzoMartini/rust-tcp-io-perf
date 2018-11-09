use std::net::TcpStream;
use std::net::SocketAddr;

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));

    if let Ok(stream) = TcpStream::connect(&addr) {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }
}
