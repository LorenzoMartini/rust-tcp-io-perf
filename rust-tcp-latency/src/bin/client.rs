extern crate bytes;
extern crate rust_tcp_latency;
extern crate streaming_harness_hdrhist;

use std::net::{Shutdown, TcpStream};
use std::io::{Read, Write};
use std::time::Instant;
use std::{thread, time};
use rust_tcp_latency::config;

fn print_line() {
    println!("\n-------------------------------------------------------------\n");
}


fn print_summary(hist: streaming_harness_hdrhist::HDRHist) {
    println!("Sent/received everything!");
    print_line();
    println!("HDRHIST summary, measure in ns");
    print_line();
    println!("summary:\n{:#?}", hist.summary().collect::<Vec<_>>());
    print_line();
    println!("Summary_string:\n{}", hist.summary_string());
    print_line();
    println!("CDF summary:\n");
    for entry in hist.ccdf() {
        println!("{:?}", entry);
    }
}


fn main() {

    let args = config::parse_config();

    println!("Connecting to the server {}...", args.address);
    let n_rounds = args.n_rounds;
    let n_bytes = args.n_kbytes * 1000;

    // Create buffers to read/write
    let wbuf = vec![0; n_bytes];
    let mut rbuf = vec![0; n_bytes];

    let progress_tracking_percentage = n_rounds / 100;

    let mut connected = false;

    while !connected {
        match TcpStream::connect(args.address_and_port()) {
            Ok(mut stream) => {
                connected = true;
                let mut hist = streaming_harness_hdrhist::HDRHist::new();
                stream.set_nodelay(true).expect("Can't set no_delay to true");
                stream.set_nonblocking(true).expect("Can't set channel to be non-blocking");

                println!("Connection established! Ready to send...");

                for i in 0..n_rounds {

                    // Make sure we send everything
                    let mut send = 0;
                    let start = Instant::now();
                    while send < n_bytes {
                        match stream.write(&wbuf) {
                            Ok(n) => send += n,
                            Err(err) => match err.kind() {
                                std::io::ErrorKind::WouldBlock => {}
                                _ => panic!("Error occurred while writing: {:?}", err),
                            }
                        }
                    }

                    // Make sure we receive the full buf back
                    let mut recv = 0;
                    while recv < n_bytes {
                        match stream.read(&mut rbuf) {
                            Ok(n) => recv += n,
                            Err(err) => match err.kind() {
                                std::io::ErrorKind::WouldBlock => {}
                                _ => panic!("Error occurred while reading: {:?}", err),
                            }
                        }
                    }

                    let duration = Instant::now().duration_since(start);
                    hist.add_value(duration.as_secs() * 1_000_000_000u64 + duration.subsec_nanos() as u64);

                    if i % progress_tracking_percentage == 0 {
                        // Track progress on screen
                        println!("{}% completed", i / progress_tracking_percentage);
                    }
                }

                stream.shutdown(Shutdown::Both).expect("shutdown call failed");

                // Format output nicely
                print_summary(hist);
            },
            Err(error) => {
                println!("Couldn't connect to server... Error {}", error);
                thread::sleep(time::Duration::from_secs(1));
            }
        }
    }
}
