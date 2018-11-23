extern crate bytes;
extern crate rust_tcp_bw;

use std::time::Instant;
use std::net::TcpListener;
use std::io::Read;
use rust_tcp_bw::config;

struct Measure {
    start: Instant,
    end: Instant,
    n_bytes: usize,
}

fn main() {
    let args = config::parse_config();
    let n_bytes = args.n_kbytes * 1000;
    if n_bytes >= 1_000_000_000 {
        panic!("OMG 1GB? this is probably too much data you wanna send")
    }
    let listener = TcpListener::bind("0.0.0.0:".to_owned() + &args.port).unwrap();

    let mut stream = listener.incoming().next().unwrap().unwrap();

    println!("Connection established with {:?}!\nExpected {} Bytes for {} rounds",
             stream.peer_addr().unwrap(), n_bytes, args.n_rounds);
    let mut buf = vec![0; n_bytes];
    let mut active = true;
    let mut measurements = Vec::new();
    let mut start = Instant::now();
    while active {
        let recv = stream.read(&mut buf).unwrap();
        if recv > 0 {
            let end = Instant::now();
            measurements.push(Measure {
                start,
                end,
                n_bytes: recv,
            });
            start = end;
        } else {
            active = false;
        }
    }

    // Print out vec of measurements, print both precise time and time in us
    println!("Done reading, results in format [N_BYTES,TIME,APPROX_TIME_IN_US]:");
    let mut tot_bytes: u64 = 0;
    let mut tot_time: u64 = 0;
    let len = measurements.len();

    for i in 0..len {
        let entry = &measurements[i];
        let duration = entry.end.duration_since(entry.start);
        let duration_us = duration.as_secs() * 1_000_000u64 + duration.subsec_micros() as u64;
        println!("[{},{:?},{}us]", entry.n_bytes, duration, duration_us);

        // Add measurement to compute bw
        if i > len / 3 && i < (len * 2 / 3) {
            tot_bytes += entry.n_bytes as u64;
            tot_time += duration_us;
        }
    }
    println!("Available approximated bandwidth: {} MB/s", tot_bytes / tot_time)
}
