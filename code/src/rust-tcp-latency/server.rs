extern crate bytes;
extern crate rust_tcp_io_perf;

use rust_tcp_io_perf::config;
use rust_tcp_io_perf::connection;
use rust_tcp_io_perf::threading;
use rust_tcp_io_perf::print_utils;

fn main() {

    let args = config::parse_config();
    let n_bytes = args.n_bytes;
    let n_rounds = args.n_rounds;
    let mut buf = vec![0; n_bytes];

    let mut stream = connection::server_listen_and_get_first_connection(&args.port);
    connection::setup(&args, &mut stream);
    threading::setup(&args);

    // Make sure n_rounds is the same between client and server
    for _i in 0..n_rounds {
        let send = connection::receive_message(n_bytes, &mut stream, &mut buf);
        let recv = connection::send_message(n_bytes, &mut stream, &buf);
        if send != 1 {
            println!("Send {}", send);
        }
        if recv != 1 {
            println!("Recv {}", recv);
        }
    }

    println!("Done exchanging stuff");
}
