extern crate bytes;
extern crate rust_tcp_ayce;

use std::time::Instant;
use std::net::TcpListener;
use std::io::Read;
use rust_tcp_ayce::config;

struct Measure {
    start: Instant,
    end: Instant,
    n_bytes: usize,
}

fn main() {
    let args = config::parse_config();
    let n_bytes = args.n_kbytes * 1000;
    let listener = TcpListener::bind("0.0.0.0:".to_owned() + &args.port).unwrap();

    let mut stream = listener.incoming().next().unwrap().unwrap();

    println!("Connection established with {:?}!\nExpected {} kB for {} rounds",
             stream.peer_addr().unwrap(), n_bytes, args.n_rounds);
    let mut buf = vec![0; n_bytes];
    let mut active = true;
    let mut measurements = Vec::new();
    while active {
        let start = Instant::now();
        let recv = stream.read(&mut buf).unwrap();
        if recv > 0 {
            let end = Instant::now();
            measurements.push(Measure {
                start,
                end,
                n_bytes: recv,
            })
        } else {
            active = false;
        }
    }

    // Print out vec of measurements, print both precise time and time in us
    println!("Done reading, results in format <N_BYTES,TIME,TIME_IN_US>:");
    for entry in measurements {
        let duration = entry.end.duration_since(entry.start);
        println!("[{},{:?},{}us]", entry.n_bytes, duration,
                 duration.as_secs() * 1_000_000u64 + duration.subsec_micros() as u64);
    }
}
