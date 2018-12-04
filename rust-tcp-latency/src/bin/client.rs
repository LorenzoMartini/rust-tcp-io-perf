extern crate bytes;
extern crate rust_tcp_latency;
extern crate streaming_harness_hdrhist;

use std::net::{Shutdown, TcpStream};
use std::time::Instant;
use std::{thread, time};
use rust_tcp_latency::config;
use rust_tcp_latency::connection;

/// Prints dashed line
fn print_line() {
    println!("\n-------------------------------------------------------------\n");
}

/// Nicely outputs summary of execution with stats and CDF points.
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
    let wbuf: Vec<u8> = vec![0; n_bytes];
    let mut rbuf: Vec<u8> = vec![0; n_bytes];

    let progress_tracking_percentage = n_rounds / 100;

    let mut connected = false;

    while !connected {
        match TcpStream::connect(args.address_and_port()) {
            Ok(mut stream) => {
                connection::setup(&args, &mut stream);
                connected = true;
                let mut hist = streaming_harness_hdrhist::HDRHist::new();

                println!("Connection established! Ready to send...");

                for i in 0..n_rounds {

                    let start = Instant::now();

                    connection::send_message(n_bytes, &mut stream, &wbuf);
                    connection::receive_message(n_bytes, &mut stream, &mut rbuf);

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
                println!("Couldn't connect to server, retrying... Error {}", error);
                thread::sleep(time::Duration::from_secs(1));
            }
        }
    }
}
