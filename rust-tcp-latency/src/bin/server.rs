extern crate bytes;
extern crate rust_tcp_latency;

use std::net::TcpListener;
use rust_tcp_latency::config;
use rust_tcp_latency::connection;

fn main() {
    let args = config::parse_config();
    let n_bytes = args.n_kbytes * 1000;
    let n_rounds = args.n_rounds;
    let mut buf = vec![0; n_bytes];

    let listener = TcpListener::bind("0.0.0.0:".to_owned() + &args.port).unwrap();
    println!("Server running, listening for connection on 0.0.0.0:{}", &args.port);

    let mut stream = listener.incoming().next().unwrap().unwrap();

    stream.set_nodelay(true).expect("Can't set no_delay to true");
    stream.set_nonblocking(true).expect("Can't set channel to be non-blocking");

    println!("Connection established with {:?}!\nExpected {} Bytes for {} rounds",
             stream.peer_addr().unwrap(), n_bytes, args.n_rounds);

    // Make sure n_rounds is the same between client and server
    for _i in 0..n_rounds {
        connection::receive_message(n_bytes, &mut stream, &mut buf);
        connection::send_message(n_bytes, &mut stream, &buf);
    }

    println!("Done exchanging stuff")
}
