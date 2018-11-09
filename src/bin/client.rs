extern crate bytes;
extern crate clap;

use clap::{Arg, App};
use std::net::{Shutdown, TcpStream};
use bytes::{BytesMut, BufMut};
use std::io::Write;

pub struct ClientConfig {
    address: String,
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
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let address = matches.value_of("address").unwrap();
    ClientConfig {
        address: String::from(address),
    }
}


fn main() {

    let args = parse_config();
    println!("N: {}", args.address);

    if let Ok(mut stream) = TcpStream::connect(args.address) {

        println!("Connected to the server!");

        let n_bytes = 1_000_000;
        let mut buf = BytesMut::with_capacity(n_bytes);
        for i in 0..n_bytes {
            buf.put_u8(1);
            if i % (n_bytes / 100) == 0 {
                println!("Progress: {} bytes loaded", i);
            }
        }

        println!("Bytes created, size: {}", buf.len());

        stream.write(&buf);
        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    } else {
        println!("Couldn't connect to server...");
    }
}
