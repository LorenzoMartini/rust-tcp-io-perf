extern crate bytes;
extern crate rust_tcp_latency;

use std::net::TcpListener;
use std::io::{Read, Write};
use rust_tcp_latency::config;

fn main() {
    let args = config::parse_config();
    let n_bytes = args.n_kbytes * 1000;
    let n_rounds = args.n_rounds;
    let mut buf = vec![0; n_bytes];

    let listener = TcpListener::bind("0.0.0.0:".to_owned() + &args.port).unwrap();

    println!("Server running, listening for connection on 0.0.0.0:{}", &args.port);
    let mut stream = listener.incoming().next().unwrap().unwrap();
    stream.set_nodelay(true).expect("Can't set no_delay to true");
    stream.set_nonblocking(true).expect("Can't set channel to be non-blocking");

    println!("Connection established with {:?}!\nExpected {} Bytes for {} rounds",
             stream.peer_addr().unwrap(), n_bytes, args.n_rounds);

    // Make sure n_rounds is the same between client and server
    for _i in 0..n_rounds {

        // Read
        let mut recv = 0;
        while recv < n_bytes {
            match stream.read(&mut buf){
                Ok(n) => recv += n,
                Err(err) => match err.kind() {
                    std::io::ErrorKind::WouldBlock => {}
                    _ => panic!("Error occurred while reading: {:?}", err),
                }
            }
        }

        // Send back
        let mut send = 0;
        while send < n_bytes {
            match stream.write(&buf) {
                Ok(n) => send += n,
                Err(err) => match err.kind() {
                    std::io::ErrorKind::WouldBlock => {}
                    _ => panic!("Error occurred while writing: {:?}", err),
                }
            }
        }
    }
    println!("Done exchanging stuff")
}
