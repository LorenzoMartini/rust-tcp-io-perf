extern crate bytes;
use std::net::TcpStream;
use std::net::SocketAddr;
use bytes::{BytesMut, BufMut};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {

    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7878") {
        println!("Connected to the server!");
        let timesize_bytes= 16;
        // 1.6GB
        let n_bytes = 1600000000;
        let n_slots = n_bytes / 16;
        let cap = n_slots;
        let mut buf = BytesMut::with_capacity(n_bytes);
        for i in 0..n_slots {
            buf.put_u128_be(1 as u128);
            if i % (1000000) == 0 {
                println!("Progress {}", i);
            }
        }
//        let epoch_time = SystemTime::now().duration_since(UNIX_EPOCH)
//            .expect("Time went backwards");
//        buf.put_u128_be((epoch_time.as_secs() * 1000 + epoch_time.subsec_nanos() as u64 / 1_000_000) as u128);
        stream.write(&buf);
    } else {
        println!("Couldn't connect to server...");
    }
}
