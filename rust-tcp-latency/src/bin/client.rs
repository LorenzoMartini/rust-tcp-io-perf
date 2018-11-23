extern crate bytes;
extern crate rust_tcp_latency;
extern crate streaming_harness_hdrhist;

use std::net::{Shutdown, TcpStream};
use std::io::{Read, Write};
use std::time::Instant;
use rust_tcp_latency::config;

fn print_line() {
    println!("\n-------------------------------------------------------------\n");
}

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
        println!("Connection established! Ready to send...");

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

        stream.shutdown(Shutdown::Both).expect("shutdown call failed");

        println!("Sent/received everything!");
        let mut hist = streaming_harness_hdrhist::HDRHist::new();
        for measure in measurements {
            hist.add_value(measure.as_secs() * 1_000_000_000u64 + measure.subsec_nanos() as u64);
        }

        // Format output nicely
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
    } else {
        println!("Couldn't connect to server...");
    }
}
