#![feature(core, io, os)]

extern crate cryptoil;
use cryptoil::sha1;
use cryptoil::stdish::io::Hasher;

pub fn digest_cmd(command: &str, message: &str) -> String {
    match command {
        "-sha1" => sha1::Sha1::new().hex_digest(message),
        _ => panic!("unknown hash algorithm"),
    }
}

pub fn main() {
    let mut reader = ::std::old_io::stdin();
    let msg: Vec<u8> = reader.read_to_end().unwrap();
    let message: &str = ::std::str::from_utf8(msg.as_slice()).unwrap();
    let args = ::std::os::args();
    let command: &str = args[1].as_slice();
    println!("{}", digest_cmd(command, message));
}
