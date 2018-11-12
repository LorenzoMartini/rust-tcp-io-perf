extern crate bytes;
extern crate clap;

use clap::{Arg, App};
use std::net::{Shutdown, TcpStream};
use std::io::Write;

pub struct ClientConfig {
    address: String,
    n_kbytes: usize,
    n_rounds: usize,
}

pub fn parse_config() -> ClientConfig {
    let matches = App::new("Client")
        .arg(Arg::with_name("address")
            .short("a")
            .long("address")
            .value_name("address")
            .help("IP4 + port to connect to, like 127.0.0.1:8000")
            .takes_value(true)
            .default_value("127.0.0.1:7878")
        )
        .arg(Arg::with_name("n_kbytes")
            .short("k")
            .long("kbytes")
            .value_name("n_kbytes")
            .help("number of kbytes to transfer, must be a multiple of 100")
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
    let address = matches.value_of("address").unwrap();
    let n_kbytes = matches.value_of("n_kbytes").unwrap().parse::<usize>().unwrap();
    let n_rounds = matches.value_of("rounds").unwrap().parse::<usize>().unwrap();
    ClientConfig {
        address: String::from(address),
        n_kbytes,
        n_rounds
    }
}


fn main() {

    let args = parse_config();

    println!("Connecting to the server {}...", args.address);
    if let Ok(mut stream) = TcpStream::connect(args.address) {
        println!("Connection established!");

        // Create a buffer of 1/100 of desired dimension and then copy it multiple times to create
        // a bigger buffer (optimization caveat)
        let n_bytes = args.n_kbytes * 1000;
        let mut buf = vec![0; n_bytes];

        println!("Ready to send...");
        for _i in 0..args.n_rounds {
            stream.write(&buf);
        }
        println!("Sent everything!");

        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    } else {
        println!("Couldn't connect to server...");
    }
}
