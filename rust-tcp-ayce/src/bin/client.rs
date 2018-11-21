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

    // Don't kill machines
    if n_bytes > 100_000_000 {
        panic!("More than 100 MB per round? this is probably too much data you wanna send, \
        you may kill one of the machines. Try with maybe 100MB but more rounds")
    }

    // Very improbable case error handling
    if n_bytes as u128 * n_rounds as u128 > u64::max_value().into() {
        panic!("There's gonna be too much data. Make sure n_bytes * n_rounds is < u128::MAX")
    }

    if let Ok(mut stream) = TcpStream::connect(args.address_and_port()) {
        println!("Connection established!");

        // Create a buffer of 1/100 of desired dimension and then copy it multiple times to create
        // a bigger buffer (optimization caveat)
        let mut buf = vec![0; n_bytes];

        let perc = n_rounds / 100;

        println!("Ready to send...");
        for i in 0..n_rounds {
            match stream.write(&buf) {
                Ok(_n) => {},
                Err(err) => panic!("crazy stuff happened while sending {}", err),
            }
            if i % perc == 0 {
                println!("{}% completed", i / perc);
            }
        }
        println!("Sent everything!");

        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    } else {
        println!("Couldn't connect to server...");
    }
}
