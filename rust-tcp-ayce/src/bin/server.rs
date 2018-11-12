extern crate bytes;
extern crate rust_tcp_ayce;

use std::net::TcpListener;
use std::io::Read;
use rust_tcp_ayce::config;

fn main() {
    let args = config::parse_config();
    let n_bytes = args.n_kbytes * 1000;
    let listener = TcpListener::bind("0.0.0.0:".to_owned() + &args.port).unwrap();

    let mut stream = listener.incoming().next().unwrap().unwrap();

    println!("Connection established with {:?}!\nExpected {} kB for {} rounds",
             stream.peer_addr().unwrap(), n_bytes, args.n_rounds);
    let mut buf = vec![0; n_bytes];
    let mut active = true;
    let mut tot = 0;
    while active {
        let recv = stream.read(&mut buf).unwrap();
        if recv > 0 {
            println!("Read {} bytes", recv);
            tot += recv;
        } else {
            active = false;
        }
    }
    println!("Done reading {} bytes", tot);
}
