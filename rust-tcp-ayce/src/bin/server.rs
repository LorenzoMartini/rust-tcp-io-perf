extern crate bytes;
extern crate clap;

use std::net::TcpListener;
use std::io::Read;
use clap::{Arg, App};

pub struct ServerConfig {
    port: String,
    n_kbytes: usize,
    n_rounds: usize,
}

pub fn parse_config() -> ServerConfig {
    let matches = App::new("Client")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("port")
            .help("port to connect to, like 7878")
            .takes_value(true)
            .default_value("7878")
        )
        .arg(Arg::with_name("n_kbytes")
            .short("k")
            .long("kbytes")
            .value_name("n_kbytes")
            .help("number of kbytes to transfer in each round, must be a multiple of 100")
            .takes_value(true)
            .default_value("10000")
        )
        .arg(Arg::with_name("rounds")
            .short("r")
            .long("rounds")
            .value_name("rounds")
            .help("number of rounds of transfer to perform")
            .takes_value(true)
            .default_value("100")
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let port = matches.value_of("port").unwrap();
    let n_kbytes = matches.value_of("n_kbytes").unwrap().parse::<usize>().unwrap();
    let n_rounds = matches.value_of("rounds").unwrap().parse::<usize>().unwrap();
    ServerConfig {
        port: String::from(port),
        n_kbytes,
        n_rounds,
    }
}


fn main() {
    let args = parse_config();
    let listener = TcpListener::bind("0.0.0.0:".to_owned() + &args.port).unwrap();

    let mut stream = listener.incoming().next().unwrap().unwrap();

    println!("Connection established with {:?}!\nExpected {} kB for {} rounds",
             stream.peer_addr().unwrap(), args.n_kbytes, args.n_rounds);
    let mut buf = vec![0; args.n_kbytes * 1000];
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
