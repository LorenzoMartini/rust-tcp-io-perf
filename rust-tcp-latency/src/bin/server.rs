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

    println!("Connection established with {:?}!\nExpected {} Bytes for {} rounds",
             stream.peer_addr().unwrap(), n_bytes, args.n_rounds);

    for _i in 0..n_rounds {
        let mut recv = 0;
        while recv < n_bytes {
            recv += stream.read(&mut buf).unwrap();
        }
        match stream.write(&buf) {
            Ok(_n) => {},
            Err(err) => panic!("crazy stuff happened while sending {}", err),
        }
    }
    println!("Done exchanging stuff")
}
