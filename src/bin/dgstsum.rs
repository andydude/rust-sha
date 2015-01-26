//use std::io::Reader;
//use std::io::stdin;
//use webclient::digest::types::HashAlgorithm;
//
//pub fn hash_algorithm_from_lower(name: &str) -> Option<Box<HashAlgorithm+'static>> {
//    match name {
//        "-sha1" => Some(Box::new(webclient::digest::sha1::sha1_new()) as Box<HashAlgorithm>),
//        _ => None
//    }
//}

fn main() {
    // get message
//    let mut reader = stdin();
//    let message: Vec<u8> = reader.read_to_end().unwrap();
//
//    // get hash algorithm
//    let args = std::os::args();
//    let command: &str = args[1].as_slice();
//    let mut hasher = hash_algorithm_from_lower(command).unwrap();
//
//    // compute hash
//    let bytes = hasher.hash(message);
//    for byte in bytes.into_iter() {
//        print!("{:02x}", byte);
//    }
    println!("");
}
