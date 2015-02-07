//use serialize::hex::ToHex;
use std::simd::u32x4;
//use std::old_io::IoError;
use super::stdish::io::{Hasher, Reset, HashRead, HashWrite};

#[unstable(feature = "default", reason = "1.0.0")]
pub mod hw;

#[unstable(feature = "default", reason = "1.0.0")]
pub mod sw;

/// Digest message block in vectors
///
/// This function
///
#[stable(feature = "default", since = "1.0.0")]
pub fn digest_block(hash: &mut [u32; 5], msg: &[u32x4; 4]) {
    //hw::digest_block(hash, msg);
    sw::digest_block(hash, msg);
}

/// Digest message block in bytes
///
/// This function
#[stable(feature = "default", since = "1.0.0")]
pub fn digest_block_bytes(hash: &mut [u8; 20], msg: &[u8]) {
    use std::slice::bytes::copy_memory;
    use super::stdish::slice::transmute_memory;
    use super::stdish::num::SwapBytesInt;
    use std::num::Int;

    assert_eq!(msg.len(), 64);
    let mut msgb = [0u8; 64];
    copy_memory(&mut msgb, msg);
    let mut msgv = [u32x4(0, 0, 0, 0); 4];
    transmute_memory::<[u32x4; 4], [u8; 64]>(&mut msgv, &msgb);
    for m in msgv.iter_mut() { *m = m.to_be() }
    let mut hashw = &mut [0u32; 5];
    transmute_memory::<[u32; 5], [u8; 20]>(&mut hashw, hash);
    for h in hashw.iter_mut() { *h = h.to_be() }

    // the crux of the algorithm
	digest_block(&mut hashw, &msgv);

    for h in hashw.iter_mut() { *h = h.to_be() }
    transmute_memory::<[u8; 20], [u32; 5]>(hash, &hashw);
}

///// Pad message
//#[unstable(feature = "default", reason = "1.0.0")]
//pub fn pad(msg: &[u8], length: usize) -> Vec<u8> {
//    let newlen = msg.len() + (-msg.len() % 64);
//    let mut bytes: Vec<u8> = Vec::with_capacity(newlen);
//    bytes.push_all(msg);
//    bytes.push(0x80u8);
//    for _ in 0us..((55 - length) % 64) {
//        bytes.push(0u8);
//    }
//    bytes.write_be_u64(8*length as u64).unwrap();
//    bytes
//}
//
//fn padding_bad() {
//    
//    // move all but last block
//    println!("{}", buf.to_hex());
//    let blockslen: usize = buf.len() + 9 + ((55 - buf.len()) % 64);
//    println!("length = {}", self.length);
//    println!("buflen = {}", buf.len());
//    println!("blockslen = {}", blockslen);
//    let blocksmostlen: usize = blockslen - 64;
//    println!("blocksmostlen = {}", blocksmostlen);
//    let mut blocksmost = [0u8; blocksmostlen];
//    
//    println!("{}", blocksmost.to_hex());
//
//    // copy last block
//    let mut lengthbuf = [0u8; 4];
//    let mut blocklast = [0u8; 128];
//    let blocklastlen: usize = buf.len() - blocksmost.len();
//    let blocklastoff: usize = blocksmostlen + blocklastlen;
//    copy_memory(blocklast, &buf[blocksmostlen..blocklastoff]);
//    blocklast[blocklastoff] = 0x80u8;
//
//    copy_memory(blocksmost, &buf[0..buf.len()]);
//    if buf.len() <= blocksmostlen {
//        blocksmost
//    } else {
//    }
//
//    let length: u32 = (8*self.length).to_u32().unwrap().to_be();
//    transmute_memory::<[u8; 4], u32>(lengthbuf, &length);
//    copy_memory(&mut blocklast[60..64], lengthbuf);
//    println!("{}", blocklast.to_hex());
//
//    //let mut bytes: Vec<u8> = Vec::with_capacity(newlen);
//    //bytes.push_all(msg);
//    //bytes.push(0x80u8);
//    //for _ in 0us..((55 - length) % 64) {
//    //    bytes.push(0u8);
//    //}
//        //bytes.write_be_u64(8*length as u64).unwrap();
//        //bytes
//    
//}

/// Pad message, 
pub fn padding(buf: &mut Vec<u8>, msglen: u64) {
    let buflen = buf.len();

    // `newlen` is always a multiple of the block size (64)
    let newlen: usize = buflen + 9 + ((55 - buflen) % 64);
    
    // standard padding
    buf.push(0x80u8);
    buf.resize(newlen - 8, 0u8);
    buf.write_be_u64(8*msglen).unwrap();
}

/// Reset hash to initial value
#[unstable(feature = "default", reason = "1.0.0")]
pub fn reset(hash: &mut [u8; 20]) {
    use std::slice::bytes::copy_memory;
    copy_memory(hash, sw::constants::SHA1_H);
}

#[stable(feature = "default", since = "1.0.0")]
#[derive(Copy, Clone, Debug)]
pub struct Sha1 {

    #[stable(feature = "default", since = "1.0.0")]
    pub finished: bool,

    #[stable(feature = "default", since = "1.0.0")]
    pub length: usize,

    #[stable(feature = "default", since = "1.0.0")]
    pub state: [u8; 20]
}

#[stable(feature = "default", since = "1.0.0")]
impl Sha1 {

    #[stable(feature = "default", since = "1.0.0")]
    pub fn new() -> Sha1 {
        Sha1 {
            finished: false,
            length: 0us,
            state: [0u8; 20]
        }
    }
}

impl Hasher for Sha1 {}

impl HashRead for Sha1 {

    /// Read a 20-byte message digest
    #[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
    fn read(&mut self, buf: &mut [u8]) {
        use std::slice::bytes::copy_memory;
        
        if buf.len() < 20us {
            panic!("Digest read buf must be exactly 20 bytes");
        }
        if !self.finished {
            panic!("Digest read requires a finished state");
        }

        // dst <- src
        copy_memory(buf, &self.state);
        
        //Ok(20us)
    }

    /// Read the message digest
    #[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) {
        buf.resize(20, 0u8);
        self.read(buf.as_mut_slice());
    }
}

impl HashWrite for Sha1 {

    /// Write a 64-byte message block
    #[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
    fn write(&mut self, buf: &[u8]) {

        if buf.len() < 64us {
            panic!("Digest write buf must be exactly 64 bytes");
        }
        if self.finished {
            panic!("Digest write requires an unfinished state");
        }

        digest_block_bytes(&mut self.state, buf);

        // the length of the last block doesn't count
        self.length +=  64us;
        
        //Ok(64us)
    }

    /// Write a bytestring message
    #[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
    fn write_all(&mut self, buf: &[u8]) {
        
        // calculate length before pad
        self.length += buf.len();
        let mut padbuf: Vec<u8> = buf.to_vec();
        padding(&mut padbuf, self.length as u64);
        let padbuflen = padbuf.len();
        
        // write remaining blocks
        for block in (&padbuf[0..(padbuflen - 64)]).chunks(64) {
            self.write(block);
        }
        self.write(&padbuf[(padbuflen - 64)..padbuflen]);
        
        // the last block increments the length
        // so we decrement it to the right value
        self.length -= 64us;
        self.finished = true;
    }
}

impl Reset for Sha1 {

    /// Reset the state to do more digests
    #[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
    fn reset(&mut self) {
        reset(&mut self.state);

        // prepare for next time
        self.finished = false;
        self.length = 0;
    }
}

//impl Hasher for Sha1 {
//    type Output = Vec<u8>;
//
//    fn reset(&mut self) {
//        reset(&mut self.state);
//
//        // prepare for next time
//        self.finished = false;
//        self.length = 0;
//    }
//    
//    fn finish(&self) -> Vec<u8> {
//        self.state.to_vec()
//    }
//}

#[cfg(test)]
pub mod tests;
