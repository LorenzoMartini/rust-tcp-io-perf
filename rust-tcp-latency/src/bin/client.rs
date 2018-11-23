extern crate bytes;
extern crate rust_tcp_latency;

use std::net::{Shutdown, TcpStream};
use std::io::{Read, Write};
use std::time::Instant;
use rust_tcp_latency::config;

fn main() {

    let args = config::parse_config();

    println!("Connecting to the server {}...", args.address);
    let n_rounds = args.n_rounds;
    let n_bytes = args.n_kbytes * 1000;
    let mut measurements = Vec::new();

    // Create buffers to read/write
    let wbuf = vec![0; n_bytes];
    let mut rbuf = vec![0; n_bytes];

    let progress_tracking_percentage = n_rounds / 100;

    if let Ok(mut stream) = TcpStream::connect(args.address_and_port()) {
        println!("Connection established!, Ready to send...");

        for i in 0..n_rounds {
            let start = Instant::now();
            match stream.write(&wbuf) {
                Ok(_n) => {},
                Err(err) => panic!("crazy stuff happened while sending {}", err),
            }

            // Make sure we receive the full buf back
            let mut recv = 0;
            while recv < n_bytes {
                recv += stream.read(&mut rbuf).unwrap();
            }
            measurements.push(Instant::now().duration_since(start));

            if i % progress_tracking_percentage == 0 {
                // Track progress on screen
                println!("{}% completed", i / progress_tracking_percentage);
            }
        }

        println!("Sent/received everything!");
        stream.shutdown(Shutdown::Both).expect("shutdown call failed");

        for measure in measurements {
            println!("{:?}, {}us", measure, measure.as_secs() * 1_000_000u64 + measure.subsec_micros() as u64);
        }
    } else {
        println!("Couldn't connect to server...");
    }
}
