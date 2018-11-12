extern crate bytes;
extern crate clap;

use std::net::TcpListener;
use std::io::Read;
use clap::{Arg, App};

pub struct ServerConfig {
    port: String,
    n_kbytes: usize,
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
            .help("number of kbytes to transfer, must be a multiple of 100")
            .takes_value(true)
            .default_value("10000")
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let port = matches.value_of("port").unwrap();
    let n_kbytes = matches.value_of("n_kbytes").unwrap().parse::<usize>().unwrap();
    ServerConfig {
        port: String::from(port),
        n_kbytes,
    }
}


fn main() {
    let args = parse_config();
    let listener = TcpListener::bind("0.0.0.0:".to_owned() + &args.port).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        println!("Connection established with {:?}!\nExpected {} kB",
                 stream.peer_addr().unwrap(), args.n_kbytes);
        let mut buf =  Vec::new();
        let mut active = true;
        while active {
            let recv = stream.read_to_end(&mut buf).unwrap();
            if recv > 0 {
                println!("Read {} bytes", recv);
            } else {
                buf.clear();
                active = false;
            }
        }
        println!("Done reading");
        return;
    }
}
