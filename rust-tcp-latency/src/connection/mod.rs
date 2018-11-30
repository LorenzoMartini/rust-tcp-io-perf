use std::net::TcpStream;
use std::io::{Read, Write};
use std::io::ErrorKind::WouldBlock;

/// Sends first n_bytes from wbuf using the given stream.
///// Make sure wbuf.len >= n_bytes
pub fn send_message(n_bytes: usize, stream: &mut TcpStream, wbuf: &Vec<u8>) {
    let mut send = 0;
    while send < n_bytes {
        match stream.write(&wbuf[send..]) {
            Ok(n) => send += n,
            Err(err) => match err.kind() {
                WouldBlock => {}
                _ => panic!("Error occurred while writing: {:?}", err),
            }
        }
    }
}

/// Reads n_bytes into rbuf from the given stream.
/// Make sure rbuf.len >= n_bytes
pub fn receive_message(n_bytes: usize, stream: &mut TcpStream, rbuf: &mut Vec<u8>) {
    // Make sure we receive the full buf back
    let mut recv = 0;
    while recv < n_bytes {
        match stream.read(&mut rbuf[recv..]) {
            Ok(n) => recv += n,
            Err(err) => match err.kind() {
                WouldBlock => {}
                _ => panic!("Error occurred while reading: {:?}", err),
            }
        }
    }
}