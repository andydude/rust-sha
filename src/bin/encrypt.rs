extern crate cryptoil;
use cryptoil::aes;

pub fn digest_cmd(command: &str, message: &str) -> String {
    match command {
        "-aes-128-ecb" => aes::hex_encrypt_ecb(message),
        _ => panic!("unknown cipher algorithm"),
    }
}

#[allow(dead_code)]
#[allow(unstable)]
fn main() {
    let mut reader = std::io::stdin();
    let msg: Vec<u8> = reader.read_to_end().unwrap();
    let message: &str = std::str::from_utf8(msg.as_slice()).unwrap();
    let args = std::os::args();
    let command: &str = args[1].as_slice();
    println!("{}", digest_cmd(command, message));
}