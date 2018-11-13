extern crate bytes;
extern crate rust_tcp_ayce;

use std::net::{Shutdown, TcpStream};
use std::io::Write;
use rust_tcp_ayce::config;

fn main() {

    let args = config::parse_config();

    println!("Connecting to the server {}...", args.address);
    let n_rounds = args.n_rounds;
    let n_bytes = args.n_kbytes * 1000;

    if n_bytes >= 1_000_000_000 {
        panic!("OMG 1GB? this is probably too much data you wanna send")
    }

    if let Ok(mut stream) = TcpStream::connect(args.address_and_port()) {
        println!("Connection established!");

        // Create a buffer of 1/100 of desired dimension and then copy it multiple times to create
        // a bigger buffer (optimization caveat)
        let mut buf = vec![0; n_bytes];

        println!("Ready to send...");
        for _i in 0..n_rounds {
            match stream.write(&buf) {
                Ok(_n) => {},
                Err(err) => panic!("crazy stuff happened while sending {}", err),
            }
        }
        println!("Sent everything!");

        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    } else {
        println!("Couldn't connect to server...");
    }
}
