extern crate core_affinity;

use std::net::TcpStream;
use std::io::{Read, Write};
use std::io::ErrorKind::WouldBlock;
use config::Config;

/// Sends first n_bytes from wbuf using the given stream.
/// Make sure wbuf.len >= n_bytes
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

/// Setup the streams and eventually pins the thread according to the configuration.
pub fn setup(config: &Config, stream: &mut TcpStream) {
    if config.no_delay {
        stream.set_nodelay(true).expect("Can't set no_delay to true");
    }
    if config.non_blocking {
        stream.set_nonblocking(true).expect("Can't set channel to be non-blocking");
    }
    if config.p_id >= 0 {
        let core_ids = core_affinity::get_core_ids().unwrap();
        core_affinity::set_for_current(core_ids[(config.p_id as usize) % core_ids.len()]);
    }
}
