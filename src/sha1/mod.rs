pub mod emu;

//#[cfg(target_arch = "arm")]
//pub mod arm;

//#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
//pub mod x86;

use serialize::hex::ToHex;
use std::simd::u32x4;

/// Digest message block in vectors
pub fn digest_block_simd(
    m0: u32x4, m4: u32x4, m8: u32x4, m12: u32x4,
    h0: u32x4, e: u32) -> (u32x4, u32) {
    emu::digest_block(m0, m4, m8, m12, h0, e)
}

/// Digest message block in words
pub fn digest_block_words(hash: &mut [u32], msg: &[u32]) {
    assert_eq!(hash.len(), 5);
    assert_eq!(msg.len(), 16);

    let h0 = u32x4(hash[0], hash[1], hash[2], hash[3]);
    let m0 = u32x4(msg[0], msg[1], msg[2], msg[3]);
    let m4 = u32x4(msg[4], msg[5], msg[6], msg[7]);
    let m8 = u32x4(msg[8], msg[9], msg[10], msg[11]);
    let m12 = u32x4(msg[12], msg[13], msg[14], msg[15]);

    let (h20, h4) = digest_block_simd(m0, m4, m8, m12, h0, hash[4]);
    let u32x4(h0, h1, h2, h3) = h20;
    
    hash[0] = h0;
    hash[1] = h1;
    hash[2] = h2;
    hash[3] = h3;
    hash[4] = h4;
}

/// Digest message block in bytes
pub fn digest_block_bytes(hash: &mut [u32], msg: &[u8]) {
    assert_eq!(hash.len(), 5);
    assert_eq!(msg.len(), 64);

    // read_u32v_be()
    let mut words: Vec<u32> = Vec::new();
    for mut word in msg.chunks(4) {
        words.push(word.read_be_u32().unwrap());
    }
    
    digest_block_words(hash, words.as_slice());
}

/// Digest whole message
pub fn digest(hash: &mut [u32], msg: &[u8]) {
    emu::init_hash(hash);
    let bytes = pad(msg, msg.len());
    for msg_block in bytes.chunks(64) {
        digest_block_bytes(hash, msg_block);
    }
}

/// Digest whole message, return hex string
pub fn hex_digest(message: &str) -> String {
    let msg = message.as_bytes();
    let mut hash = [0u32; 5];
    digest(&mut hash, msg);
    
    // write_u32v_be()
    let mut bytes: Vec<u8> = Vec::new();
    for t in 0us..5us {
        bytes.write_be_u32(hash[t]).unwrap();
    }
    
    bytes.to_hex()
}

/// Pad message
pub fn pad(msg: &[u8], length: usize) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.push_all(msg);
    bytes.push(0x80u8);
    for _ in 0us..((55 - length) % 64) {
        bytes.push(0u8);
    }
    bytes.write_be_u64(8*length as u64).unwrap();
    bytes
}

#[cfg(test)]
pub mod tests;
