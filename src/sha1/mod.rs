//use serialize::hex::ToHex;
use std::simd::u32x4;
use std::num::Int;
use std::old_io::IoError;
use std::slice::bytes::copy_memory;
use super::stdish::slice::transmute_memory;
use super::stdish::io::{Digest, Reset, Read, Write, io_error};
use super::stdish::num::SwapBytesInt;

pub mod emu;

//#[cfg(target_arch = "arm")]
//pub mod arm;
//
//#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
//pub mod x86;
//
///// Digest message block in vectors (arm-specific)
//#[cfg(target_arch = "arm")]
//#[unstable(feature = "cryptoil", reason = "std::simd is unstable")]
//pub fn digest_block_simd(msg_0: u32x4,
//    msg_16: u32x4, msg_32: u32x4, msg_48: u32x4,
//    hash_abcd: u32x4, hash_e: u32) -> (u32x4, u32) {
//    if arm::has_sha() {
//        arm::digest_block_simd(msg_0, msg_16, msg_32, msg_48, hash_abcd, hash_e)
//    } else {
//        emu::digest_block_simd(msg_0, msg_16, msg_32, msg_48, hash_abcd, hash_e)
//    }
//}
//
//
///// Digest message block in vectors (x86-specific)
//#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
//#[unstable(feature = "cryptoil", reason = "std::simd is unstable")]
//pub fn digest_block_simd(msg_0: u32x4,
//    msg_16: u32x4, msg_32: u32x4, msg_48: u32x4,
//    hash_abcd: u32x4, hash_e: u32) -> (u32x4, u32) {
//    if x86::has_sha() {
//        x86::digest_block_simd(msg_0, msg_16, msg_32, msg_48, hash_abcd, hash_e)
//    } else {
//        emu::digest_block_simd(msg_0, msg_16, msg_32, msg_48, hash_abcd, hash_e)
//    }
//}

/// Digest message block in vectors
//#[cfg(not(any(target_arch = "arm", target_arch = "x86", target_arch = "x86_64")))]
#[unstable(feature = "cryptoil", reason = "std::simd is unstable")]
pub fn digest_block_simd(hashw: &mut [u32; 5], msgv: &[u32x4; 4]) {
    emu::digest_block_simd(hashw, msgv);
}

/// Digest message block in bytes
#[stable(feature = "cryptoil", since = "1.0.0")]
pub fn digest_block_bytes(hash: &mut [u8; 20], msg: &[u8]) {
    assert_eq!(msg.len(), 64);
    let mut msgb: [u8; 64] = [0u8; 64];
    copy_memory(&mut msgb, msg);
    let mut msgv: [u32x4; 4] = [u32x4(0, 0, 0, 0); 4];
    transmute_memory::<[u32x4; 4], [u8; 64]>(&mut msgv, &msgb);
    for m in msgv.iter_mut() { *m = m.to_be() }
    let mut hashw: [u32; 5] = [0u32; 5];
    transmute_memory::<[u32; 5], [u8; 20]>(&mut hashw, hash);
    for h in hashw.iter_mut() { *h = h.to_be() }
	digest_block_simd(&mut hashw, &msgv);
    for h in hashw.iter_mut() { *h = h.to_be() }
    transmute_memory::<[u8; 20], [u32; 5]>(hash, &hashw);
}

#[unstable(feature = "cryptoil", reason = "will be trait method")]
#[derive(Copy, Clone, Debug)]
pub struct Sha1 {
    pub finished: bool,
    pub length: usize,
    state: [u8; 20]
}

impl Digest<IoError> for Sha1 {}

#[unstable(feature = "cryptoil", reason = "will be trait method")]
impl Sha1 {
    pub fn new() -> Sha1 {
        Sha1 {
            finished: false,
            length: 0us,
            state: [0u8; 20]
        }
    }
}

#[unstable(feature = "cryptoil", reason = "std::old_io and std::io are both unstable")]
impl Read<IoError> for Sha1 {

    /// Read a 20-byte message digest
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        //println!("{} in read", self.state.to_hex());
        if buf.len() < 20us {
            return Err(io_error("Digest read buf must be exactly 20 bytes"))
        }
        if !self.finished {
            return Err(io_error("Digest read requires a finished state"))
        }
        
        copy_memory(buf, &self.state);
        
        Ok(20us)
    }

    /// Read the message digest
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<(), IoError> {
        //println!("{} in read_to_end", self.state.to_hex());
        buf.resize(20, 0u8);
        try!(self.read(buf.as_mut_slice()));
        Ok(())
    }
}

#[unstable(feature = "cryptoil", reason = "std::old_io and std::io are both unstable")]
impl Write<IoError> for Sha1 {

    /// Write a 64-byte message block
    fn write(&mut self, buf: &[u8]) -> Result<usize, IoError> {
        //println!("{} in write", self.state.to_hex());
        if buf.len() < 64us {
            return Err(io_error("Digest write buf must be exactly 64 bytes"))
        }
        if self.finished {
            return Err(io_error("Digest write requires an unfinished state"))
        }

        digest_block_bytes(&mut self.state, buf);

        // the length of the last block doesn't count
        self.length += 64us;
        Ok(64us)
    }

    /// Write a bytestring message
    fn write_all(&mut self, buf: &[u8]) -> Result<(), IoError> {
        //println!("{} in write_all", self.state.to_hex());
        self.length += buf.len();
        let bytes = emu::pad(buf, self.length);
        for block in bytes.chunks(64) {
            try!(self.write(block));
        }
        self.length -= 64us;
        self.finished = true;
        Ok(())
    }
}

#[unstable(feature = "cryptoil", reason = "std::hash::Hasher is unstable")]
impl Reset for Sha1 {

    /// Reset the state to do more digests
    fn reset(&mut self) {
        copy_memory(&mut self.state, emu::SHA1_H);

        // prepare for next time
        self.finished = false;
        self.length = 0;
    }
}

/// Digest whole message, return hex string
#[unstable(feature = "cryptoil", reason = "will be trait method")]
pub fn hex_digest(msg: &str) -> String {
    Sha1::new().hex_digest(msg).unwrap()
}

#[cfg(test)]
pub mod tests;
