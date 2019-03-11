extern crate bytes;
extern crate rust_tcp_io_perf;
extern crate hdrhist;
extern crate amd64_timer;

use std::{thread, time};
use rust_tcp_io_perf::config;
use rust_tcp_io_perf::connection;
use rust_tcp_io_perf::print_utils;
use rust_tcp_io_perf::threading;
use amd64_timer::ticks;

fn main() {

    let args = config::parse_config();

    println!("Connecting to the server {}...", args.address);
    let n_rounds = args.n_rounds;
    let n_bytes = args.n_bytes;

    // Create buffers to read/write
    let wbuf: Vec<u8> = vec![0; n_bytes];
    let mut rbuf: Vec<u8> = vec![0; n_bytes];

    let progress_tracking_percentage = (n_rounds * 2) / 100;

    let mut connected = false;

    while !connected {
        match connection::client_connect(args.address_and_port()) {
            Ok(mut stream) => {
                connection::setup(&args, &mut stream);
                threading::setup(&args);
                connected = true;
                let mut hist = hdrhist::HDRHist::new();
                let mut hist_read = hdrhist::HDRHist::new();
                let mut hist_write = hdrhist::HDRHist::new();

                println!("Connection established! Ready to send...");

                // To avoid TCP slowstart we do double iterations and measure only the second half
                for i in 0..(n_rounds * 2) {

                    let t0 = ticks();

                    let write_duration = connection::send_message(n_bytes, &mut stream, &wbuf);
                    let read_duration = connection::receive_message(n_bytes, &mut stream, &mut rbuf);

                    let t1 = ticks();

                    if i >= n_rounds {
                        hist.add_value(t1 - t0);
                        hist_read.add_value(read_duration);
                        hist_write.add_value(write_duration);
                    }

                    if i % progress_tracking_percentage == 0 {
                        // Track progress on screen
                        println!("{}% completed", i / progress_tracking_percentage);
                    }
                }
                connection::close_connection(&stream);
                print_utils::print_summary(hist);
                println!("\n--- WRITE ---");
                print_utils::print_summary(hist_write);
                println!("\n--- READ ---");
                print_utils::print_summary(hist_read);
            },
            Err(error) => {
                println!("Couldn't connect to server, retrying... Error {}", error);
                thread::sleep(time::Duration::from_secs(1));
            }
        }
    }
}
