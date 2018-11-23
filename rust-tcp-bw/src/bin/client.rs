extern crate bytes;
extern crate rust_tcp_bw;

use std::net::{Shutdown, TcpStream};
use std::io::Write;
use rust_tcp_bw::config;

fn main() {

    let args = config::parse_config();

    println!("Connecting to the server {}...", args.address);
    let n_rounds = args.n_rounds;
    let n_bytes = args.n_kbytes * 1000;

    if let Ok(mut stream) = TcpStream::connect(args.address_and_port()) {
        println!("Connection established! Ready to send...");

        // Create a buffer of 0s, size n_bytes, to be sent over multiple times
        let mut buf = vec![0; n_bytes];
        let progress_tracking_percentage = n_rounds / 100;

        for i in 0..n_rounds {
            match stream.write(&buf) {
                Ok(_n) => {},
                Err(err) => panic!("crazy stuff happened while sending {}", err),
            }
            if i % progress_tracking_percentage == 0 {
                println!("{}% completed", i / progress_tracking_percentage);
            }
        }
        println!("Sent everything!");

        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    } else {
        println!("Couldn't connect to server...");
    }
}
