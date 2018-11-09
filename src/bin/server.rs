use std::net::TcpListener;
use std::io::Read;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        println!("Connection established!");
        let mut active = true;
        while active {
            let mut buf =  Vec::new();
            let recv = stream.read_to_end(&mut buf).unwrap();
            println!("Read {} bytes", recv);
            //active = false;
        }
    }
}
