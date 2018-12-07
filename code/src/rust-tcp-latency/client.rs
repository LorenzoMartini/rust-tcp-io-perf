extern crate bytes;
extern crate rust_tcp_io_perf;
extern crate streaming_harness_hdrhist;

use std::time::Instant;
use std::{thread, time};
use rust_tcp_io_perf::config;
use rust_tcp_io_perf::connection;
use rust_tcp_io_perf::print_utils;
use rust_tcp_io_perf::threading;

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
        match connection::client_connect(args.address_and_port()) {
            Ok(mut stream) => {
                connection::setup(&args, &mut stream);
                threading::setup(&args);
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
                connection::close_connection(&stream);
                print_utils::print_summary(hist);
            },
            Err(error) => {
                println!("Couldn't connect to server, retrying... Error {}", error);
                thread::sleep(time::Duration::from_secs(1));
            }
        }
    }
}
